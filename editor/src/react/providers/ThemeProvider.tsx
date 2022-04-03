import {MantineProvider} from '@mantine/core';
import type {FC} from 'react';

import {useReduxSelector} from '../../store/store';

export const ThemeProvider: FC = ({children}) => {
    const theme = useReduxSelector(state => state.theme.theme);

    return <MantineProvider theme={theme}>{children}</MantineProvider>;
};
