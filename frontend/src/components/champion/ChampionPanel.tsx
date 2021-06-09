import React from "react";
import { DropTargetMonitor, useDrop } from "react-dnd";
import { CHAMPION } from "../../constants/constants";
import Action from "../../models/Action";
import Team from "../../models/Team";
import useStore from "../../store/DraftStore";
import BottomIcon from "../svg/roles/BottomIcon";
import TopIcon from "../svg/roles/TopIcon";

const DDRAGON_URL = process.env.NEXT_PUBLIC_DDRAGON_URL;

interface IProps {
    team: Team;
    type: Action;
    championId: string | null;
    index: number;
    onClick: () => void;
}

const ChampionPanel: React.FC<IProps> = ({
    team,
    type,
    championId,
    index,
    onClick,
}) => {
    const isPick = type === Action.Pick;
    const isAlly = team === Team.Ally;

    const championData = useStore((store) => store.championData);

    const champion = championId != null ? championData[championId] : null;

    const [{ canDrop, isOver }, drop] = useDrop(
        () => ({
            accept: CHAMPION,
            drop: () => ({ team, type, index }),
            collect: (monitor: DropTargetMonitor) => ({
                isOver: monitor.isOver(),
                canDrop: monitor.canDrop(),
            }),
        }),
        [team, type, index]
    );

    const isActive = canDrop && isOver;

    return (
        <div
            ref={drop}
            className={`h-full ${
                isAlly ? "border-r-2" : "border-l-2"
            } border-b-2 border-gray-700 py-3 px-4 cursor-pointer ${
                isActive ? "bg-gray-700" : ""
            }`}
            onClick={onClick}
        >
            <h3 className="font-header uppercase text-xl mb-1">
                {isPick ? "Pick" : "Ban"} {index + 1}
            </h3>
            {champion && (
                <div className="flex justify-between">
                    <div className="flex flex-col">
                        <div className="relative w-16 h-16 min-w-16 min-h-16 overflow-hidden mr-4">
                            <img
                                src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${champion.id}.png`}
                                className={`w-16 h-16 min-w-16 min-h-16 absolute inset-0 ${
                                    isPick ? "" : "filter grayscale"
                                }`}
                                style={{ transform: "scale(1.1,1.1)" }}
                            ></img>
                        </div>
                        <h4 className="text-2xl font-header">
                            {champion.name}
                        </h4>
                    </div>
                    {isPick && (
                        <>
                            <div className="flex flex-col items-center">
                                <span className="block text-xs font-medium uppercase tracking-wider text-gray-400">
                                    Role
                                </span>
                                <div className="flex space-x-2">
                                    <div className="flex flex-col items-center">
                                        <BottomIcon className="w-10" />
                                        <span className="block text-xs font-medium uppercase tracking-wider text-gray-400">
                                            68%
                                        </span>
                                    </div>

                                    <div className="flex flex-col items-center">
                                        <BottomIcon className="w-10" />
                                        <span className="block text-xs font-medium uppercase tracking-wider text-gray-400">
                                            68%
                                        </span>
                                    </div>
                                </div>
                            </div>
                            <div>
                                <span className="block text-xs font-medium uppercase tracking-wider text-gray-400">
                                    Player
                                </span>
                            </div>
                        </>
                    )}
                </div>
            )}
        </div>
    );
};

export default ChampionPanel;
