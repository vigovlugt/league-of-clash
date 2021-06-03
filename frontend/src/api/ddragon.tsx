import IChampion from "../models/IChampion";

const DDRAGON_URL = process.env.NEXT_PUBLIC_DDRAGON_URL;

export async function getVersion() {
    const res = await fetch(DDRAGON_URL + "api/versions.json");
    const json = await res.json();

    return json[0];
}

export async function getChampions(version?: string): Promise<{
    [key: number]: IChampion;
}> {
    if (!version) {
        version = await getVersion();
    }

    const res = await fetch(
        DDRAGON_URL + "cdn/" + version + "/data/en_US/champion.json"
    );
    const json = await res.json();

    return Object.values(json.data).reduce(
        (acc: any, c: any) => ({
            ...acc,
            [c.key]: { id: c.id, name: c.name, key: c.key },
        }),
        {}
    ) as { [key: number]: IChampion };
}
