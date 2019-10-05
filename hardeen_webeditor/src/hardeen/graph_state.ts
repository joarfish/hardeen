import { HardeenNodeModel } from "../node-graph/nodes/HardeenNodeModel";
import { HardeenGraphPath } from "../../../hardeen_wasm/pkg/hardeen_wasm";
import { AppState } from "../app-state/AppState";
import { SwitchToSubgraph, SwitchToGraphPath } from "../app-state/Messenger";
import { DiagramModel, DiagramEngine } from "@projectstorm/react-diagrams";

interface GraphState {
    outputNode: HardeenNodeModel,
    model: Object,
}

export const trackGraphStates = (model: DiagramModel, engine: DiagramEngine, appState: AppState) => {
    let graphStates : Map<string, GraphState> = new Map();

    appState.messenger.subscribe("SwitchToSubgraph", (message: SwitchToSubgraph) => {

        const hc = appState.hardeenCore;
        let subgraphPath = null;
        let parentPath = appState.currentGraphPath;

        if(message.node=="root") {
            subgraphPath = hc.get_root_path();
        }
        else {
            subgraphPath = hc.get_graph_path(parentPath, message.node.getHardeenHandle())
        }

        const lastModel = {
			outputNode: appState.outputNode,
			model: model.serialize()
		};

		graphStates.set(hc.hash_graph_path(parentPath), lastModel);

		appState.currentGraphPath = subgraphPath;
		let subgraphState = graphStates.get(hc.hash_graph_path(subgraphPath));

		if(subgraphState!=undefined) {
			model.deserializeModel(subgraphState.model as any, engine);
		}
		else {
			subgraphState = {
				outputNode: null,
				model: new DiagramModel()
			}
			
			model.deserializeModel((subgraphState.model as DiagramModel).serialize(), engine);
			graphStates.set(hc.hash_graph_path(subgraphPath), subgraphState);
		}

		appState.selectedNode = null;

        appState.messenger.send({
            type: "SetOutputNode",
            node: subgraphState.outputNode
        });

        appState.messenger.send({
            type: "SwitchedToSubgraph",
            parent_path: parentPath
        });

		engine.repaintCanvas();

    });


    appState.messenger.subscribe("SwitchToGraphPath", (message: SwitchToGraphPath) => {

        const hc = appState.hardeenCore;
        let newGraphPath = null;
        let lastPath = appState.currentGraphPath;

        if(message.path=="root") {
            newGraphPath = hc.get_root_path();
        }
        else {
            newGraphPath = message.path;
        }

        const lastModel = {
			outputNode: appState.outputNode,
			model: model.serialize()
		};

		graphStates.set(hc.hash_graph_path(lastPath), lastModel);

		appState.currentGraphPath = newGraphPath;
		let graphState = graphStates.get(hc.hash_graph_path(newGraphPath));

		if(graphState!=undefined) {
			model.deserializeModel(graphState.model as any, engine);
		}
		else {
            console.error("Error: Untracked Graph Selected!");
		}

		appState.selectedNode = null;

        appState.messenger.send({
            type: "SetOutputNode",
            node: graphState.outputNode
        });

		engine.repaintCanvas();

    });

}