import { readFile } from "fs/promises";

enum TileType {
    VERTICAL = '┃',
    HORIZONTAL = '━',
    NORTH_EAST_BEND = '┗',
    NORTH_WEST_BEND = '┛',
    SOUTH_WEST_BEND = '┓',
    SOUTH_EAST_BEND = '┏',
    GROUND = '▒',
    START = '╳'
}

class Position {
    public x: number;
    public y: number;

    constructor(x: number = 0, y: number = 0) {
        this.x = x;
        this.y = y;
    }

    public eq(other: Position): boolean {
        return (this.x === other.x) && (this.y === other.y);
    }

    public neq(other: Position): boolean {
        return !this.eq(other);
    }
}



class MazeTile {
    static readonly CHAR_TILE_MAP = new Map([
        ['|', TileType.VERTICAL],
        ['-', TileType.HORIZONTAL],
        ['L', TileType.NORTH_EAST_BEND],
        ['J', TileType.NORTH_WEST_BEND],
        ['7', TileType.SOUTH_WEST_BEND],
        ['F', TileType.SOUTH_EAST_BEND],
        ['.', TileType.GROUND],
        ['S', TileType.START]
    ]);

    static readonly PRINT_CHAR_MAP = new Map([
        [TileType.VERTICAL, '┃'],
        [TileType.HORIZONTAL, '━'],
        [TileType.NORTH_EAST_BEND, '┗'],
        [TileType.NORTH_WEST_BEND, '┛'],
        [TileType.SOUTH_WEST_BEND, '┓'],
        [TileType.SOUTH_EAST_BEND, '┏'],
        [TileType.GROUND, '░'],
        [TileType.START, '╳']
    ]);

    private type: TileType;
    private isStart: boolean;
    private connectedToStart: boolean;
    private position: Position;

    /**
     * Construct a new MazeTile instance from a given character
     * @param c a string containing a single character
     */
    constructor(c: string, x: number, y: number) {
        this.type = MazeTile.CHAR_TILE_MAP.get(c) ?? TileType.GROUND;
        this.isStart = this.type == TileType.START;
        this.connectedToStart = this.isStart;
        this.position = new Position(x, y);
    }

    public toString(): string {
        const bkgColor: number = this.isStart ? 44 : 40;
        const foregroundColor: number = this.connectedToStart ? 33 : 37;
        const tileColor = `\x1b[${foregroundColor};${bkgColor}m`;

        const tileChar = MazeTile.PRINT_CHAR_MAP.get(this.type) ?? '?';

        return `${tileColor}${tileChar}\x1b[0`;
    }

    public static printTiles(tiles: Array<Array<MazeTile>>) {
        let outStr = "";
        for (const row of tiles) {
            for (const tile of row) {
                outStr += tile.toString();
            }
            outStr += '\n';
        }
        console.log(outStr);
    }

    private determinePipeType(tiles: Array<Array<MazeTile>>) {
        if (this.type != TileType.START) {
            return;
        }
        const pos = this.position;

        const prevRow = pos.y - 1;
        const nextRow = pos.y + 1;
        const prevCol = pos.x - 1;
        const nextCol = pos.x + 1;

        const northPipeType = prevRow >= 0 ? tiles[prevRow][pos.x].type : null;
        const southPipeType = nextRow < tiles.length ? tiles[nextRow][pos.x].type : null;
        const westPipeType = prevCol >= 0 ? tiles[pos.y][prevCol].type : null;
        const eastPipeType = nextCol < tiles[pos.y].length ? tiles[pos.y][nextCol].type : null;

        const connectsNorth: boolean = northPipeType != null &&
            [TileType.VERTICAL, TileType.SOUTH_WEST_BEND, TileType.SOUTH_EAST_BEND].includes(northPipeType);
        const connectsSouth: boolean = southPipeType != null &&
            [TileType.VERTICAL, TileType.NORTH_WEST_BEND, TileType.NORTH_EAST_BEND].includes(southPipeType);
        const connectsEast: boolean = eastPipeType != null &&
            [TileType.HORIZONTAL, TileType.SOUTH_WEST_BEND, TileType.NORTH_WEST_BEND].includes(eastPipeType);
        const connectsWest: boolean = westPipeType != null &&
            [TileType.HORIZONTAL, TileType.SOUTH_EAST_BEND, TileType.NORTH_EAST_BEND].includes(westPipeType);

        let startType = TileType.START;
        if (connectsNorth && connectsSouth) {
            startType = TileType.VERTICAL;
        } else if (connectsEast && connectsWest) {
            startType = TileType.HORIZONTAL;
        } else if (connectsNorth && connectsEast) {
            startType = TileType.NORTH_EAST_BEND;
        } else if (connectsNorth && connectsWest) {
            startType = TileType.NORTH_WEST_BEND;
        } else if (connectsSouth && connectsEast) {
            startType = TileType.SOUTH_EAST_BEND;
        } else if (connectsSouth && connectsWest) {
            startType = TileType.SOUTH_WEST_BEND;
        }

        this.type = startType;
    }

    /**
     * Locates starting tile, determines it's type, and returns its position
     * @param tiles 
     * @returns starting tile's position
     */
    public static setupStartTile(tiles: Array<Array<MazeTile>>): MazeTile | null {
        let tile = null;
        for (let y = 0; y < tiles.length; y++) {
            for (let x = 0; x < tiles[y].length; x++) {
                const tmp = tiles[y][x];
                if (tmp.type == TileType.START) {
                    tile = tmp;
                }
            }
        }

        if (tile) {
            // Determine type of start pipe
            tile.determinePipeType(tiles);
        }

        return tile;
    }

    private getConnections(): { con0: Position, con1: Position } {
        const type = this.type;
        const pos = this.position;
        let con0 = new Position();
        let con1 = new Position();
        switch (type) {
            case TileType.HORIZONTAL:
                con0 = new Position(pos.x - 1, pos.y);
                con1 = new Position(pos.x + 1, pos.y);
                break;
            case TileType.VERTICAL:
                con0 = new Position(pos.x, pos.y - 1);
                con1 = new Position(pos.x, pos.y + 1);
                break;
            case TileType.NORTH_EAST_BEND:
                con0 = new Position(pos.x, pos.y - 1);
                con1 = new Position(pos.x + 1, pos.y);
                break;
            case TileType.NORTH_WEST_BEND:
                con0 = new Position(pos.x, pos.y - 1);
                con1 = new Position(pos.x - 1, pos.y);
                break;
            case TileType.SOUTH_EAST_BEND:
                con0 = new Position(pos.x, pos.y + 1);
                con1 = new Position(pos.x + 1, pos.y);
                break;
            case TileType.SOUTH_WEST_BEND:
                con0 = new Position(pos.x, pos.y + 1);
                con1 = new Position(pos.x - 1, pos.y);
                break;
        }

        return { con0, con1 };
    }

    public static traversePipes(tiles: Array<Array<MazeTile>>, startTile: MazeTile): number {
        let connections = startTile.getConnections();

        let prev = startTile;
        let next = tiles[connections.con0.y][connections.con0.x];
        next.connectedToStart = true;

        let stepCount = 0;
        while (next.position.neq(startTile.position)) {
            stepCount++;

            connections = next.getConnections();
            let tmpTile = tiles[connections.con0.y][connections.con0.x];
            if (tmpTile === prev) {
                tmpTile = tiles[connections.con1.y][connections.con1.x];
            }
            prev = next;
            next = tmpTile;
            next.connectedToStart = true;
        }
        return Math.ceil(stepCount / 2);
    }
}

const TEST_INPUT: string = `7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
`;

async function main() {
    const input_str = await readFile("input.txt", { encoding: 'utf-8' });
    const tiles: Array<Array<MazeTile>> = [];
    const lines = input_str.split(/\n\r?/);
    for (let y = 0; y < lines.length; y++) {
        const row: MazeTile[] = [];
        const chars = lines[y].split('');
        for (let x = 0; x < chars.length; x++) {
            const c = chars[x];
            row.push(new MazeTile(c, x, y));
        }
        tiles.push(row);
    }

    const startTile = MazeTile.setupStartTile(tiles);
    if (!startTile) {
        throw new Error("Failed to find staring pipe");
    }

    const furthestPt = MazeTile.traversePipes(tiles, startTile);
    MazeTile.printTiles(tiles);
    console.log(`Part 1 result: ${furthestPt}`);
}

main()
    .catch((reason) => {
        console.error(reason);
    });
