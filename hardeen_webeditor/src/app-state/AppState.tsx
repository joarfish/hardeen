import {observable, computed, decorate} from "mobx";
import { NodeType, HardeenCoreInterface, GeometryWorld, HardeenHandle, HardeenGraphPath } from '../../../hardeen_wasm/pkg';
import Messenger from "./Messenger";
import { HardeenNodeModel } from "../node-graph/nodes/HardeenNodeModel";

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

    inputFocused: boolean; // This is to prevent React-Diagram from taking over the input to our property editor fields (especially Backspace and Del)
}

decorate(AppState, {
    allNodeTypes: observable,
    hardeenCore: observable,
    renderOutput: observable,
    selectedNode: observable,
    outputNode: observable,
    currentGraphPath: observable,
    inputFocused: observable,
    nodeType: computed,
});