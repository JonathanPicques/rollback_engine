import {createSlice} from '@reduxjs/toolkit';
import type {PayloadAction} from '@reduxjs/toolkit';
import type {MantineThemeOverride} from '@mantine/core';

export const themeSlice = createSlice({
    name: 'theme',
    initialState: {
        theme: {colorScheme: 'dark'} as MantineThemeOverride,
    },
    reducers: {
        setTheme: (state, action: PayloadAction<MantineThemeOverride>) => {
            state.theme = action.payload;
        },
    },
});

export const {setTheme} = themeSlice.actions;
