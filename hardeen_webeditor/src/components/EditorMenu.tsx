/** @jsx jsx */

import {NodeType} from "../../../hardeen_wasm/pkg"
import {css, jsx} from "@emotion/core";
import Messenger from "../app-state/Messenger";

interface EditorMenuProps {
    nodeTypes: NodeType[];
    messenger: Messenger;
}

const EditorMenuStyle = css`
    padding: 1rem;
    z-index: 1;
    background-color: #222222;
    grid-area: graphMenu;
    color: white;
`;

const SubmenuStyle = css`
    z-index: 2;
    position: absolute;
    background-color: grey;
    list-style: none;
    max-height: 0;
    overflow: hidden;
    > li {
        cursor: pointer;
        padding: 0.5rem;
    }
    li:hover {
        background-color: blue;
        color: white;
    }
`;

const MenuEntryStyle = css`
    list-style: none;
    > li {
        position: relative;
        display: inline-block;
        padding: 0 1rem;
        > ul {
            ${SubmenuStyle}
        }
    }
    > li:hover {
        cursor: pointer;
        text-decoration: underline;
        > ul {
            max-height: 1000px;
        }
    }
`;

const EditorMenu = (props: EditorMenuProps) => {

    return <div css={EditorMenuStyle}>
        <ul css={MenuEntryStyle}>
            <li onClick={() => props.messenger.send({type:"SaveAll"})}>
                File
            </li>
            <li>Create Node
                <ul>
                    {
                        props.nodeTypes.map((nodeType) => (<li key={nodeType.name} onClick={() => props.messenger.send({type:"CreateNode", nodeType: nodeType})}>{nodeType.name}</li>))
                    }
                </ul>
            </li>
            <li>
                About
            </li>
            <li>
                <span onClick={
                    () => {
                        props.messenger.send({type: "MoveLevelUp"});
                    }
                }>Level up</span>
            </li>
        </ul>
    </div>
};

export default EditorMenu;