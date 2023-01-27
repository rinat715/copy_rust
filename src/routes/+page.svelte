<script>
    import { Tabs } from '@svelteuidev/core';
    import { Camera, Gear } from 'radix-icons-svelte';
    import Worklogs from './worklogs.svelte';
</script>

<Tabs color="lime">
    <Tabs.Tab label='Worklogs' icon={Camera}><Worklogs/></Tabs.Tab>
    <Tabs.Tab label='Settings' icon={Gear}>Settings tab content</Tabs.Tab>
</Tabs>