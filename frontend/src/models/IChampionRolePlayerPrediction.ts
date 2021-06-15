import Role from "./Role";

interface IChampionRolePlayerPrediction {
    champion_id: number;
    probability: number;
    role: Role;
    summoner_name: string;
}

export default IChampionRolePlayerPrediction;
