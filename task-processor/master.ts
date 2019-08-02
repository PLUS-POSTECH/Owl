import * as cluster from "cluster"
import { synchdFn } from "synchd"
import { prisma, Task } from "../generated/prisma-client"
import { Message, MessageType } from "./types"

async function fetchTask() {
  const unscheduledTasks: Task[] = await prisma
    .tasks({
      where: {
        status: "PENDING"
      },
      orderBy: "createdAt_ASC"
    });

  if (unscheduledTasks.length === 0) return null

  const targetTask = unscheduledTasks[0]
  await prisma.updateTask({
    where: {
      id: targetTask.id
    },
    data: {
      status: "EXPLOITING"
    }
  });

  return targetTask.id
}

const fetchLock = {};
const exclusiveTaskFetch = synchdFn(fetchLock, fetchTask);

async function assignWorker(worker: cluster.Worker) {
  const taskId = await exclusiveTaskFetch()
  if (taskId === null) {
    worker.send(new Message(MessageType.Sleep, null))
  } else {
    worker.send(new Message(MessageType.TaskPush, taskId))
  }
}

function wakeWorker() {
  const workers = cluster.workers
  for (const id in workers) {
    workers[id]!.send(new Message(MessageType.Wakeup, null))
  }
}

function watchTasks(subscription: AsyncIterator<Task>) {
  subscription.next().then((_) => {
    wakeWorker()
    watchTasks(subscription)
  })
}

export async function run() {
  const workers = cluster.workers
  for (const id in workers) {
    const worker = workers[id]!
    worker.on("message", (_) => assignWorker(worker))
  }

  const newTaskIterator = await prisma.$subscribe.task({ mutation_in: ["CREATED"] }).node()
  watchTasks(newTaskIterator)
}
