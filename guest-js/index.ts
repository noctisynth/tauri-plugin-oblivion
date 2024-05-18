import { invoke } from "@tauri-apps/api/core";

interface ClientInstance {
  uuid: string;
  entrance: string;
  message: string;
  success: boolean;
}

interface ResponseData {
  header: string;
  content: Uint8Array;
  entrance: string;
  status_code: number;
  flag: number;
}

class Response implements ResponseData {
  header: string;
  content: Uint8Array;
  entrance: string;
  status_code: number;
  flag: number;

  constructor(response: ResponseData) {
    this.header = response.header;
    this.content = new Uint8Array(response.content);
    this.entrance = response.entrance;
    this.status_code = response.status_code;
    this.flag = response.flag;
  }

  get text() {
    return new TextDecoder().decode(this.content);
  }

  get json() {
    return JSON.parse(this.text);
  }
}

class Client {
  closed: boolean = false;

  constructor(public uuid: string | null, public entrance: string) {
    this.uuid = uuid;
    this.entrance = entrance;
  }

  async connect(entrance: string | null = null): Promise<void> {
    if (!entrance && !this.entrance) {
      throw new Error("An oblivion entrance is required");
    }
    const result: ClientInstance = await invoke("plugin:oblivion|connect", {
      entrance: entrance || this.entrance,
    });
    this.uuid = result.uuid;
  }

  async close(): Promise<void> {
    await invoke("plugin:oblivion|close", { uuid: this.uuid });
    this.closed = true;
  }

  async send_json(data: any): Promise<boolean> {
    const result: boolean = await invoke("plugin:oblivion|send_json", {
      uuid: this.uuid,
      data,
    });
    return result;
  }

  async recv(): Promise<Response> {
    const result: ResponseData = await invoke("plugin:oblivion|recv", {
      uuid: this.uuid,
    });
    return new Response(result);
  }
}

export async function connect(entrance: string): Promise<Client> {
  let client: ClientInstance = await invoke("plugin:oblivion|connect", {
    entrance,
  });
  return new Client(client.uuid, entrance);
}
