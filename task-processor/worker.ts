import * as fs from "mz/fs"
import * as path from "path"
import { execFile } from "mz/child_process"
import { dir as tmpdir } from "tmp-promise"
import { toByteArray } from "base64-js"
import { prisma, Task, TaskStatus } from "../generated/prisma-client"
import { Message, MessageType } from "./types"


let isIdle = false

async function updateStatus(taskId: number, status: TaskStatus, message: string) {
  await prisma.updateTask({
    data: {
      status: status,
      statusMessage: message
    },
    where: { id: taskId }
  })
}

async function finalizeExploit(taskId: number, stdout: string, stderr: string) {
  await prisma.updateTask({
    data: {
      status: "SUBMITTING",
      exploitStdout: stdout,
      exploitStderr: stderr
    },
    where: { id: taskId }
  })
}

async function runExploit(task: Task) {
  const endpoint = await prisma.task({ id: task.id }).endpoint()
  const exploit = await prisma.task({ id: task.id }).exploit()

  const exploitAttachmentOption = exploit.attachment
  if (exploitAttachmentOption === undefined)
    updateStatus(task.id, "EXPLOIT_ERROR", "Nothing to run.")

  const encodedAttachment = exploitAttachmentOption!
  const decodedAttachment = toByteArray(encodedAttachment)

  const exploitDir = await tmpdir()
  const exploitName = "exploit.py"
  const exploitPath = path.join(exploitDir.path, exploitName)

  await fs.writeFile(exploitPath, Buffer.from(decodedAttachment))
  await fs.chmod(exploitPath, 0o644)

  const [exploitStdout, exploitStderr] = await execFile("python", [exploitPath, endpoint.connectionString])
  await finalizeExploit(task.id, exploitStdout, exploitStderr)

  // TODO: Implement submit and handle errors

  await exploitDir.cleanup()
}

async function runSubmit(task: Task) {

}

async function runTask(taskId: number) {
  const taskOption = await prisma.task({ id: taskId })
  if (taskOption === null) return

  const task = taskOption!
  await runExploit(task)
  await runSubmit(task)
}

async function handleMessage(message: Message) {
  switch (message.type) {
    case MessageType.Sleep: {
      isIdle = true
    }
    case MessageType.TaskPush: {
      await runTask(message.message as number)
      process.send!(new Message(MessageType.TaskRequest, null))
    }
    case MessageType.Wakeup: {
      if (isIdle === true) {
        isIdle = false
        process.send!(new Message(MessageType.TaskRequest, null))
      }
    }
  }
}

export function run() {
  process.on("message", handleMessage)
}
