import {IntlProvider} from 'react-intl';
import type {FC} from 'react';

import {useReduxSelector} from '../../store/store';

export const LocaleProvider: FC = ({children}) => {
    const locale = useReduxSelector(state => state.locale.locale);

    return (
        <IntlProvider locale={locale} messages={{}}>
            {children}
        </IntlProvider>
    );
};
