import {observable, computed, decorate} from "mobx";
import { NodeType, HardeenCoreInterface, GeometryWorld, HardeenHandle, HardeenGraphPath } from '../../../hardeen_wasm/pkg';
import Messenger from "./Messenger";
import { HardeenNodeModel } from "../hardeen-nodes/HardeenNodeModel";

export class AppState {

    allNodeTypes: [NodeType];

    get nodeType() : { [key: string] : NodeType } {
        let typesToIndex : { [key: string] : NodeType } = {};

        for(let i=0; i < this.allNodeTypes.length; i++) {
            const nodeType = this.allNodeTypes[i];
            typesToIndex[nodeType.name] = nodeType;
        }

        return typesToIndex;
    }

    hardeenCore: HardeenCoreInterface;
    messenger: Messenger;
    renderOutput : GeometryWorld;
    selectedNode : HardeenHandle;
    outputNode: HardeenNodeModel;
    currentGraphPath: HardeenGraphPath;
}

decorate(AppState, {
    allNodeTypes: observable,
    hardeenCore: observable,
    renderOutput: observable,
    selectedNode: observable,
    outputNode: observable,
    currentGraphPath: observable,
    nodeType: computed,
});