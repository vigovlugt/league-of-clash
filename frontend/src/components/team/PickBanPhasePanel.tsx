import React, { useState } from "react";
import Action from "../../models/Action";
import { BAN_PHASE_1, PICK_PHASE_1, PICK_PHASE_2 } from "../../models/Phase";
import Team from "../../models/Team";
import useStore from "../../store/DraftStore";
import ChampionPanel from "../champion/ChampionPanel";
import { ChampionSelect } from "../champion/ChampionSelect";
import ChampionSelectPanel from "../champion/ChampionSelectPanel";

interface IProps {
    team: Team;
}

const PickBanPhasePanel: React.FC<IProps> = ({ team }) => {
    const isAllyTeam = team === Team.Ally;

    const phase = useStore((store) => store.phase);
    const isPickPhase = [PICK_PHASE_1, PICK_PHASE_2].includes(phase);

    const bans = useStore(
        (store) => store[isAllyTeam ? "allyBans" : "enemyBans"]
    );
    const setBan = useStore(
        (store) => store[isAllyTeam ? "setAllyBan" : "setEnemyBan"]
    );

    const picks = useStore(
        (store) => store[isAllyTeam ? "allyPicks" : "enemyPicks"]
    );
    const setPick = useStore(
        (store) => store[isAllyTeam ? "setAllyPick" : "setEnemyPick"]
    );

    const [selectedIndex, setSelectedIndex] = useState<number | null>(null);

    const onClickChampionPanel = (i: number) => {
        setSelectedIndex(i);
    };

    const onCloseChampionSelect = () => {
        setSelectedIndex(null);
    };

    const onSelectChampion = (id: string) => {
        if (isPickPhase) {
            setPick(selectedIndex as number, id);
        } else {
            setBan(selectedIndex as number, id);
        }
        setSelectedIndex(null);
    };

    if (selectedIndex != null) {
        return (
            <ChampionSelectPanel
                onSelect={onSelectChampion}
                onClose={onCloseChampionSelect}
            ></ChampionSelectPanel>
        );
    }

    return (
        <div className="h-full overflow-auto flex flex-col">
            <h2
                className={`px-4 py-3 font-header text-4xl text-primary border-b-2 border-gray-700 ${
                    isAllyTeam ? "border-r-2" : "border-l-2"
                }`}
            >
                <span className="uppercase">
                    {isAllyTeam ? "Ally" : "Enemy"}{" "}
                    {isPickPhase ? "Picks" : "Bans"}
                </span>
            </h2>
            <div className="flex flex-col h-full">
                {[...new Array(5)].map((_, i) => (
                    <ChampionPanel
                        key={i}
                        type={isPickPhase ? Action.Pick : Action.Ban}
                        championId={isPickPhase ? picks[i] : bans[i]}
                        team={team}
                        index={i}
                        onClick={() => onClickChampionPanel(i)}
                    ></ChampionPanel>
                ))}
            </div>
        </div>
    );
};

export default PickBanPhasePanel;
