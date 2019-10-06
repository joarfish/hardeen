import * as React from 'react';
import * as ReactDOM from 'react-dom';
import './main.css';
import createEngine, { DiagramModel, DefaultDiagramState } from '@projectstorm/react-diagrams';
import { HardeenNodeFactory } from './node-graph/nodes/HardeenNodeFactory';
import { HardeenNodeModel } from './node-graph/nodes/HardeenNodeModel';
import { HardeenWebeditor } from './HardeenWebeditor';
import {AppState} from "./app-state/AppState";
import Messenger, {CreateNode, SaveAll, CreateLink, DeleteLink, SetOutputNode, NodeSelected, DeleteNode, SubgraphNodeSelected} from "./app-state/Messenger";
import {action, autorun} from "mobx";
import {SlottedInputPort} from "./node-graph/ports/SlottedInputPort";
import PortFactory from './node-graph/ports/PortFactory';
import { registerMessageHandler } from './hardeen/hardeen';
import { trackGraphStates } from './hardeen/graph_state';
import { Action, InputType } from '@projectstorm/react-canvas-core';



import(/* webpackChunkName: "hardeen" */ "../../hardeen_wasm/pkg" ).then( (hardeen) => {

	const appState = new AppState();

	appState.messenger = new Messenger();
	appState.hardeenCore = hardeen.HardeenCoreInterface.new();
	appState.allNodeTypes = appState.hardeenCore.get_node_types();

	console.log("Node Types:");
	console.log(appState.allNodeTypes);

	const engine = createEngine();

	engine.getNodeFactories().registerFactory(new HardeenNodeFactory(appState.messenger));
	engine.getPortFactories().registerFactory(new PortFactory('output-port'));
	engine.getPortFactories().registerFactory(new PortFactory('input-port-slotted'));
	engine.getPortFactories().registerFactory(new PortFactory('input-port-multiple'));

	
	const state = engine.getStateMachine().getCurrentState();
	if (state instanceof DefaultDiagramState) {
		state.dragNewLink.config.allowLooseLinks = false;

		// This is a bugfix to prevent React-Diagram from taking over inputs to property editor's input fields. 
		const actions = engine.getActionEventBus().getActionsForType(InputType.KEY_DOWN);
		autorun(() => {
			if(appState.inputFocused) {
				actions.forEach( action => engine.getActionEventBus().deregisterAction(action) );
			}
			else {
				actions.forEach( action => engine.getActionEventBus().registerAction(action) );
			}
		});
	}



	const model = new DiagramModel();

	appState.currentGraphPath = appState.hardeenCore.get_root_path();

	const handle = appState.hardeenCore.add_processor_node(appState.currentGraphPath,"Empty");
	const node = new HardeenNodeModel({hardeenHandle: handle, nodeType: appState.allNodeTypes[0], isSubgraphProcessor: false});
	node.setPosition(50,50);
	model.addNode(node);

	engine.setModel(model);

	model.registerListener({
		nodesUpdated: action((event) => {
			if(!event.isCreated && event.node instanceof HardeenNodeModel) {
				appState.messenger.send({
					type: "DeleteNode",
					hdNodeHandle: event.node.hardeenHandle
				});
			}
		} ),
		eventDidFire: action((event) => {

			// @ts-ignore
			if(event.function == "linksUpdated" && event.isCreated && event.link.targetPort==null) {
				// @ts-ignore
				event.link.registerListener({
					targetPortChanged: action(linkEvent => {
						let msg = {
							type: "CreateLink",
							// @ts-ignore
							from: linkEvent.entity.sourcePort.getNode().getHardeenHandle(),
							// @ts-ignore
							to: linkEvent.entity.targetPort.getNode().getHardeenHandle()
						};
						// @ts-ignore
						if(event.link.targetPort instanceof SlottedInputPort) {
							// @ts-ignore
							msg.slot = event.link.targetPort.getOptions().slotNumber;
						}

						appState.messenger.send(msg);
					})
				});
			}
			// @ts-ignore
			else if(event.function == "linksUpdated" && !event.isCreated && event.link.targetPort!=null) {
				let msg : DeleteLink = {
					type: "DeleteLink",
					// @ts-ignore
					from: event.link.sourcePort.getNode().getHardeenHandle(),
					// @ts-ignore
					to: event.link.targetPort.getNode().getHardeenHandle()
				};

				// @ts-ignore
				if(event.link.targetPort instanceof SlottedInputPort) {
					// @ts-ignore
					msg.slot = event.link.targetPort.getOptions().slotNumber;
				}

				appState.messenger.send(msg);
			}
		})
	});

	registerMessageHandler(model, engine, appState);
	trackGraphStates(model, engine, appState);

	ReactDOM.render(<HardeenWebeditor engine={engine} appState={appState} />, document.querySelector('#application'));

});


