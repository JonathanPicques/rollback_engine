import {createSlice} from '@reduxjs/toolkit';
import type {PayloadAction} from '@reduxjs/toolkit';

export enum Locales {
    fr = 'fr',
    en = 'en',
}

export const localeSlice = createSlice({
    name: 'locale',
    initialState: {
        locale: Locales.fr,
    },
    reducers: {
        setLocale: (state, action: PayloadAction<Locales>) => {
            state.locale = action.payload;
        },
    },
});

export const {setLocale} = localeSlice.actions;
