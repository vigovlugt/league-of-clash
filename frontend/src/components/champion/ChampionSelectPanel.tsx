import React, { useRef, useEffect, useState, FormEvent } from "react";
import useStore from "../../store/DraftStore";
import BackIcon from "../svg/BackIcon";
import ChampionOption from "./ChampionOption";

interface IProps {
    onSelect: (id: number) => void;
    onClose: () => void;
}

const ChampionSelectPanel: React.FC<IProps> = ({ onSelect, onClose }) => {
    const championData = useStore((store) => store.championData);
    const pickBannedChampions = useStore(
        (store) => store.getPickBannedChampions
    )();

    const input = useRef(null);
    const [query, setQuery] = useState("");

    useEffect(() => {
        if (input.current != null) {
            (input.current as any).focus();
        }
    }, [input]);

    const champions = Object.values(championData).filter(
        (c) =>
            !pickBannedChampions.includes(+c.key) &&
            (!query || c.name.toLowerCase().includes(query.toLowerCase()))
    );

    const onSubmit = (e: FormEvent) => {
        e.preventDefault();

        if (champions.length == 0) return;
        onSelect(+champions[0].key);
    };

    return (
        <div className="h-full overflow-auto flex flex-col">
            <div className="flex p-4 justify-between">
                <button onClick={onClose} className="w-8">
                    <BackIcon className="w-8" />
                </button>
                <h2 className="font-header text-4xl text-primary uppercase">
                    Select champion
                </h2>
                <div className="w-8" />
            </div>

            <form onSubmit={onSubmit} className="w-full px-3">
                <input
                    className="px-3 py-2 bg-light-dark rounded-md mb-2 w-full"
                    ref={input}
                    value={query}
                    onChange={(e) => setQuery(e.target.value)}
                    placeholder="Search champion"
                ></input>
            </form>

            <div className="flex flex-col h-full">
                {champions.map((c) => (
                    <ChampionOption
                        key={c.key}
                        champion={c}
                        onClick={() => onSelect(+c.key)}
                    />
                ))}
            </div>
        </div>
    );
};

export default ChampionSelectPanel;
