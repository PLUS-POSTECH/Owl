export enum MessageType {
  Sleep,
  TaskPush,
  TaskRequest,
  Wakeup,
}

export interface Message {
  type: MessageType
  message?: any
}
