import {HardeenHandle, NodeType} from "../../../hardeen_wasm/pkg";
import { HardeenNodeModel } from "../hardeen-nodes/HardeenNodeModel";

export interface CreateNode {
    type: "CreateNode";
    nodeType: NodeType;
}

export interface NodeCreated {
    type: "NodeCreated";
    hdNodeHandle: HardeenHandle;
}

export interface DeleteNode {
    type: "DeleteNode";
    hdNodeHandle: HardeenHandle;
}

export interface CreateLink {
    type: "CreateLink",
    from: HardeenHandle,
    to: HardeenHandle,
    port?: number
}

export interface DeleteLink {
    type: "DeleteLink",
    from: HardeenHandle,
    to: HardeenHandle,
    port?: number
}

export interface SaveAll {
    type: "SaveAll";
}

export interface SetOutputNode {
    type: "SetOutputNode";
    node: HardeenNodeModel;
}

export interface NodeSelected {
    type: "NodeSelected";
    node: HardeenNodeModel;
}

export interface SubgraphNodeSelected {
    type: "SubgraphNodeSelected";
    node: HardeenNodeModel;
}

export interface MoveLevelUp {
    type: "MoveLevelUp"
}


type MessageType = "CreateNode" | "DeleteNode" | "CreateLink" | "DeleteLink" | "SaveAll" | "SetOutputNode" | "NodeSelected" | "SubgraphNodeSelected" | "MoveLevelUp";
type Message = CreateNode | DeleteNode | NodeCreated | CreateLink | DeleteLink | SaveAll | SetOutputNode | NodeSelected | SubgraphNodeSelected | MoveLevelUp;

export default class Messenger {

    subscriptions: { [key in MessageType]?: Set<Function> };

    constructor() {
        this.subscriptions = {};
    }

    send(message: Message) {
        if(this.subscriptions.hasOwnProperty(message.type)) {
            this.subscriptions[message.type].forEach( (handler) => {
                handler(message);
            } )
        }
    }

    subscribe(type: MessageType, handler: Function) {
        if(!this.subscriptions.hasOwnProperty(type)) {
            this.subscriptions[type] = new Set();
        }
        this.subscriptions[type].add(handler);
    }

    unsubscribe(type: MessageType, handler: Function) {
        this.subscriptions[type].delete(handler);
    }

}