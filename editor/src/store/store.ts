import undoable from 'redux-undo';
import {configureStore} from '@reduxjs/toolkit';
import {useDispatch, useSelector} from 'react-redux';
import type {TypedUseSelectorHook} from 'react-redux';

import {appSlice} from './app/app';
import {themeSlice} from './theme/theme';
import {localeSlice} from './locale/locale';

export const store = configureStore({
    reducer: {
        app: undoable(appSlice.reducer),
        theme: themeSlice.reducer,
        locale: localeSlice.reducer,
    },
});

export type ReduxState = ReturnType<typeof store.getState>;
export type ReduxDispatch = typeof store.dispatch;

export const useReduxDispatch = () => useDispatch<ReduxDispatch>();
export const useReduxSelector: TypedUseSelectorHook<ReduxState> = useSelector;
