export enum MessageType {
  Sleep,
  TaskPush,
  TaskRequest,
  Wakeup,
}

export class Message {
  type: MessageType
  message: any
  constructor(type: MessageType, message: any) {
    this.type = type
    this.message = message
  }
}
