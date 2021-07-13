import { Popover } from "@headlessui/react";
import { useState } from "react";
import { usePopper } from "react-popper";
import IChampion from "../../models/IChampion";
import Team from "../../models/Team";
import {
    getWinrate,
    winrateBorderClass,
    winrateClass,
} from "../../utils/winrate";

const DDRAGON_URL = process.env.NEXT_PUBLIC_DDRAGON_URL;

interface IProps {
    champion: IChampion;
    iconSize: string;
    team: Team;
}

const ChampionInfo: React.FC<IProps> = ({ champion, iconSize, team }) => {
    const [referenceElement, setReferenceElement] = useState();
    const [popperElement, setPopperElement] = useState();
    const { styles, attributes } = usePopper(referenceElement, popperElement, {
        placement: "right",
        modifiers: [
            {
                name: "offset",
                options: {
                    offset: [0, 8],
                },
            },
        ],
    });

    const isAlly = team === Team.Ally;

    return (
        <Popover className="relative">
            <Popover.Button
                ref={setReferenceElement as any}
                className="flex items-center z-0"
            >
                <div className={`relative overflow-hidden mr-4  ${iconSize}`}>
                    <img
                        src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${champion.id}.png`}
                        className={`absolute inset-0 ${iconSize}`}
                        style={{ transform: "scale(1.1,1.1)" }}
                    ></img>
                </div>
                <h4 className="text-xl font-header">{champion.name}</h4>
            </Popover.Button>

            <Popover.Panel
                ref={setPopperElement as any}
                style={styles.popper}
                {...attributes.popper}
                className="absolute z-10 bg-darker rounded-xl p-4 flex flex-col items-center text-gray-50 cursor-default"
            >
                <a
                    href={`https://u.gg/lol/champions/${champion.id}/build`}
                    target="_blank"
                    rel="noreferrer"
                    className="mb-2"
                >
                    <h4 className="text-2xl font-header text-center">
                        {champion.name}
                    </h4>
                </a>
                <span className="block text-xs font-medium uppercase tracking-wider text-gray-400 mb-1">
                    {isAlly ? "Enemy" : "Ally"} Counters
                </span>
                <div className="flex space-x-2">
                    <ChampionIcon champion={champion} />
                    <ChampionIcon champion={champion} />
                    <ChampionIcon champion={champion} />
                    <ChampionIcon champion={champion} />
                </div>
                <span className="block text-xs font-medium uppercase tracking-wider text-gray-400 mt-3 mb-1">
                    {isAlly ? "Ally" : "Enemy"} Duos
                </span>
                <div className="flex space-x-2">
                    <ChampionIcon champion={champion} />
                    <ChampionIcon champion={champion} />
                    <ChampionIcon champion={champion} />
                    <ChampionIcon champion={champion} />
                </div>
            </Popover.Panel>
        </Popover>
    );
};

interface IChampionIconProps {
    champion: IChampion;
    wins?: number;
    games?: number;
}

const ChampionIcon: React.FC<IChampionIconProps> = ({
    champion,
    wins = 10,
    games = 20,
}) => {
    wins = Math.random() * 10 + 5;

    return (
        <div className="flex flex-col items-center">
            <div
                className={`relative overflow-hidden w-12 h-12 min-w-12 min-h-12 rounded-full border-[3px] ${winrateBorderClass(
                    wins,
                    games
                )} box-content`}
            >
                <img
                    src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${champion.id}.png`}
                    className={`absolute inset-0 w-12 h-12 min-w-12 min-h-12`}
                    style={{ transform: "scale3d(1.1,1.1,1.1)" }}
                ></img>
            </div>
            <span
                className={`block text-xs font-medium uppercase tracking-wider ${winrateClass(
                    wins,
                    games
                )}`}
            >
                {getWinrate(wins, games)}%
            </span>
        </div>
    );
};

export default ChampionInfo;
