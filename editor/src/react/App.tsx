import {Provider} from 'react-redux';
import {DockLayout} from 'rc-dock';
import {useLocalStorage} from 'react-use';
import {Center, useMantineTheme} from '@mantine/core';
import {Terminal2, FileCode, Stack2, Photo, Folders, Activity, Adjustments} from 'tabler-icons-react';
import type {VFC} from 'react';
import type {TabBase, TabData, LayoutData} from 'rc-dock';

import 'rc-dock/dist/rc-dock-dark.css';

import {Run} from './Run';
import {Tree} from './Tree';
import {store} from '../store/store';
import {Script} from './Script';
import {ThemeProvider} from './providers/ThemeProvider';
import {LocaleProvider} from './providers/LocaleProvider';

const defaultTab: TabData = {
    title: '',
    group: 'default',
    content: <></>,
    closable: true,
};

const defaultLayout: LayoutData = {
    dockbox: {
        mode: 'horizontal',
        children: [
            {
                mode: 'vertical',
                size: 100,
                children: [
                    {
                        tabs: [{...defaultTab, id: 'tree'}],
                    },
                    {
                        tabs: [{...defaultTab, id: 'files'}],
                    },
                ],
            },
            {
                mode: 'vertical',
                children: [
                    {
                        tabs: [
                            {...defaultTab, id: 'scene::mario'},
                            {...defaultTab, id: 'script::mario'},
                            {...defaultTab, id: 'script::goomba'},
                        ],
                    },
                ],
            },
            {
                mode: 'vertical',
                size: 100,
                children: [
                    {
                        tabs: [{...defaultTab, id: 'inspector'}],
                    },
                ],
            },
        ],
    },
};

const defaultTabRenderers = {
    _: {icon: Activity, component: () => null},
    run: {icon: Terminal2, component: Run},
    tree: {icon: Stack2, component: Tree},
    scene: {icon: Photo, component: Tree},
    files: {icon: Folders, component: Tree},
    script: {
        icon: FileCode,
        component: function ScriptEditor({name}: {name: string}) {
            const [script, setScript] = useLocalStorage(name, '');

            return <Script name={name} value={script || ''} onChange={setScript} />;
        },
    },
    inspector: {icon: Adjustments, component: Tree},
};

const loadTab = (tab: TabBase & {id: keyof typeof defaultTabRenderers}): TabData => {
    const tabRenderer = defaultTabRenderers[(tab.id! || '').split('::')[0]! as keyof typeof defaultTabRenderers] || defaultTabRenderers._;

    return {
        ...defaultTab,
        id: tab.id,
        title: (
            <Center inline>
                <tabRenderer.icon size={18} />
                <span>{tab.id || defaultTab.title}</span>
            </Center>
        ),
        content: <tabRenderer.component name={tab.id} />,
    };
};

const AppContent = () => {
    const theme = useMantineTheme();

    return (
        <div
            style={{
                flex: 1,
                display: 'flex',
                flexDirection: 'column',
                backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0],
            }}
        >
            <Center style={{padding: 5}}>
                <Run />
            </Center>
            <DockLayout
                style={{flex: 1}}
                dropMode="edge"
                //
                groups={{default: {animated: false, floatable: false}}}
                loadTab={loadTab}
                defaultLayout={defaultLayout}
            />
        </div>
    );
};

export const App: VFC = () => {
    return (
        <Provider store={store}>
            <LocaleProvider>
                <ThemeProvider>
                    <AppContent />
                </ThemeProvider>
            </LocaleProvider>
        </Provider>
    );
};
