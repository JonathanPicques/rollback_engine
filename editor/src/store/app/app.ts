import {createSlice} from '@reduxjs/toolkit';

type AppFileName = SceneAppFileName | ScriptAppFileName;
type SceneAppFileName = `scene://${string}`;
type ScriptAppFileName = `script://${string}`;

interface AppFile {
    name: AppFileName;
}

interface AppState {
    files: {
        names: AppFileName[];
        byName: Record<AppFileName, AppFile>;
    };
}

const initialState: AppState = {
    files: {
        names: [],
        byName: {},
    },
};

export const appSlice = createSlice({
    name: 'app',
    reducers: {},
    initialState,
});
