import {Group, Button} from '@mantine/core';
import {PlayerPlay, PlayerStop, Package} from 'tabler-icons-react';
import type {VFC} from 'react';

export const Run: VFC = () => {
    return (
        <Group>
            <Button size="xs" color="green" title="ds" variant="outline">
                <PlayerPlay strokeWidth={1.5} />
            </Button>
            <Button size="xs" color="red" variant="outline">
                <PlayerStop strokeWidth={1.5} />
            </Button>
            <Button size="xs" color="teal" variant="outline">
                <Package strokeWidth={1.5} />
            </Button>
        </Group>
    );
};
