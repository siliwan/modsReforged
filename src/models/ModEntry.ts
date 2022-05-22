import { ServerData } from './ServerData'

export class ServerModEntry {
    modId: string
    name: string
    version: string

    constructor (data: ServerData) {
      this.modId = data.id
      this.name = data.name
      this.version = data.revision.version
    }
}
