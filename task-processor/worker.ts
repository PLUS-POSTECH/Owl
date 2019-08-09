import execa, { ExecaError, ExecaReturnValue } from "execa";
import * as fs from "fs-extra";
import * as path from "path";
import { EOL } from "os";
import { dir as tmpdir } from "tmp-promise";
import { toByteArray } from "base64-js";
import { prisma, Task, TaskStatus } from "../generated/prisma-client";
import { Message, MessageType } from "./types";

let isIdle = false;

async function updateExploitOutputs(
  taskId: number,
  stdout: string,
  stderr: string
) {
  await prisma.updateTask({
    data: {
      exploitStdout: stdout,
      exploitStderr: stderr
    },
    where: { id: taskId }
  });
}

async function updateSubmitOutputs(
  taskId: number,
  stdout: string,
  stderr: string
) {
  await prisma.updateTask({
    data: {
      submitStdout: stdout,
      submitStderr: stderr
    },
    where: { id: taskId }
  });
}

async function updateStatus(taskId: number, status: TaskStatus, message = "") {
  await prisma.updateTask({
    data: {
      status: status,
      statusMessage: message
    },
    where: { id: taskId }
  });
}

async function finalizeExploit(taskId: number, result: ExecaReturnValue) {
  const stdout = result.stdout;
  const stderr = result.stderr;
  await updateExploitOutputs(taskId, stdout, stderr);

  const flag = stderr.split(EOL).pop();
  await prisma.updateTask({
    data: {
      status: "SUBMITTING",
      flag: flag
    },
    where: { id: taskId }
  });
}

async function finalizeExploitError(taskId: number, result: ExecaError) {
  const exitCode = result.exitCode;
  const timedOut = result.timedOut;
  const incompatible = exitCode === 64;

  const message = result.message;
  const stdout = result.stdout;
  const stderr = result.stderr;
  await updateExploitOutputs(taskId, stdout, stderr);

  if (incompatible) {
    await updateStatus(taskId, "EXPLOIT_INCOMPATIBLE");
  } else if (timedOut) {
    await updateStatus(taskId, "EXPLOIT_TIMEOUT");
  } else {
    await updateStatus(taskId, "EXPLOIT_ERROR", message);
  }
}

async function finalizeSubmit(taskId: number, result: ExecaReturnValue) {
  const stdout = result.stdout;
  const stderr = result.stderr;
  await updateSubmitOutputs(taskId, stdout, stderr);

  const submitResult = stderr.split(EOL).pop();
  if (submitResult === "CORRECT") {
    await updateStatus(taskId, "SUBMIT_CORRECT");
  } else if (submitResult === "WRONG") {
    await updateStatus(taskId, "SUBMIT_WRONG");
  } else if (submitResult === "DUPLICATE") {
    await updateStatus(taskId, "SUBMIT_DUPLICATE");
  } else {
    await updateStatus(
      taskId,
      "SUBMIT_ERROR",
      `Unknown status output: ${submitResult}`
    );
  }
}

async function finalizeSubmitError(taskId: number, result: ExecaError) {
  const message = result.message;
  const stdout = result.stdout;
  const stderr = result.stderr;
  await updateSubmitOutputs(taskId, stdout, stderr);
  await updateStatus(taskId, "SUBMIT_ERROR", message);
}

async function runExploit(task: Task) {
  const endpoint = await prisma.task({ id: task.id }).endpoint();
  const exploit = await prisma.task({ id: task.id }).exploit();

  const exploitAttachment = exploit.attachment;
  if (exploitAttachment === undefined) {
    updateStatus(task.id, "EXPLOIT_ERROR", "Nothing to run.");
    return;
  }

  const decodedAttachment = toByteArray(exploitAttachment);
  const exploitDir = await tmpdir();
  const exploitName = "exploit.py";
  const exploitPath = path.join(exploitDir.path, exploitName);

  await fs.writeFile(exploitPath, Buffer.from(decodedAttachment));
  await fs.chmod(exploitPath, 0o644);

  const exploitExecOptions = { timeout: 0 };
  let exploitResult: ExecaReturnValue;

  try {
    exploitResult = await execa(
      "python",
      [exploitPath, endpoint.connectionString],
      exploitExecOptions
    );
  } catch (error) {
    await finalizeExploitError(task.id, error);
    return;
  } finally {
    await exploitDir.cleanup();
  }
  await finalizeExploit(task.id, exploitResult);
}

async function runSubmit(task: Task) {
  const submitName = "submit.py";
  const submitPath = path.join(process.cwd(), submitName);
  const flag = task.flag;

  if (flag === undefined) {
    return;
  } else if (flag === "SKIP_SUBMIT") {
    await updateStatus(task.id, "SUBMIT_SKIPPED");
    return;
  }

  await fs.chmod(submitPath, 0o644);

  let submitResult: ExecaReturnValue;
  try {
    submitResult = await execa("python", [submitPath, flag]);
  } catch (error) {
    await finalizeSubmitError(task.id, error);
    return;
  }

  await finalizeSubmit(task.id, submitResult);
}

async function runTask(taskId: number) {
  try {
    const task = await prisma.task({ id: taskId });
    if (task === null) return;
    await runExploit(task);

    const updatedTask = await prisma.task({ id: taskId });
    if (updatedTask === null) return;
    await runSubmit(updatedTask);
  } catch (error) {
    await updateStatus(taskId, "UNKNOWN_ERROR", error.message);
  }
}

async function handleMessage(message: Message) {
  const taskRequestMessage: Message = { type: MessageType.TaskRequest };
  switch (message.type) {
    case MessageType.Sleep: {
      isIdle = true;
      break;
    }
    case MessageType.TaskPush: {
      await runTask(message.message! as number);
      process.send!(taskRequestMessage);
      break;
    }
    case MessageType.Wakeup: {
      if (isIdle === true) {
        isIdle = false;
        process.send!(taskRequestMessage);
      }
      break;
    }
  }
}

export function run() {
  process.on("message", handleMessage);
}
