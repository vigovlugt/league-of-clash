import React from "react";
import Select from "react-select";
import useStore from "../../store/DraftStore";

interface IProps {
    championId: number | null;
    setChampion: (champion: string) => void;
    placeholder: string;
}

export const ChampionSelect: React.FC<IProps> = ({
    championId: champion,
    setChampion,
    placeholder,
}) => {
    const championData = useStore((store) => store.championData);
    const pickBannedChampions = useStore(
        (store) => store.getPickBannedChampions
    )();

    const options = Object.values(championData).map((c) => ({
        value: +c.key,
        label: c.name,
    }));

    const selectedOption = options.find((o) => o.value == champion);

    return (
        <Select
            className="mb-4 max-w-sm"
            value={selectedOption}
            onChange={(o) => o && setChampion(championData[o.value].key)}
            options={options}
            placeholder={placeholder}
            isOptionDisabled={(option) =>
                pickBannedChampions.includes(option.value)
            }
            theme={(theme) => ({
                ...theme,
                colors: {
                    ...theme.colors,
                    neutral0: "#333333",
                    neutral20: "transparent",
                    neutral80: "white",
                    primary25: "#262626",
                    primary: "#D0A755",
                },
            })}
            styles={{
                option: (styles, { isDisabled, isSelected }) => ({
                    ...styles,
                    color: isSelected
                        ? "white"
                        : isDisabled
                        ? "hsl(0, 0%, 50%);"
                        : "",
                }),
            }}
        />
    );
};
