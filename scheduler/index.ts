import { synchdFn } from 'synchd';
import { prisma, Task, TaskRunner } from "../generated/prisma-client";

const oneMinuteInMilliseconds = 60 * 1000;

async function schedule() {
  const unscheduledTasks: Task[] = await prisma
    .tasks({
      where: {
        runner: null
      },
      orderBy: "createdAt_ASC"
    });

  const idleRunners: TaskRunner[] = await prisma
    .taskRunners({
      where: {
        idle: true
      },
      orderBy: "updatedAt_ASC"
    });

  const schedulingCount = Math.min(unscheduledTasks.length, idleRunners.length);

  for (let i = 0; i < schedulingCount; i++) {
    const task = unscheduledTasks[i];
    const runner = idleRunners[i];
    await prisma.updateTask({
      where: {
        id: task.id
      },
      data: {
        runner: {
          connect: {
            id: runner.id
          }
        }
      }
    });
    await prisma.updateTaskRunner({
      where: {
        id: runner.id
      },
      data: {
        idle: false
      }
    });
  }
}

const scheduleLock = {};
const exclusiveSchedule = synchdFn(scheduleLock, schedule);

// Maybe use subscribe?
setInterval(exclusiveSchedule, oneMinuteInMilliseconds);
