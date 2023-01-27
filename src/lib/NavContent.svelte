<script>
    import { Tabs } from '@svelteuidev/core';
    import { Camera, EnvelopeClosed, Gear } from 'radix-icons-svelte';
</script>

<Tabs>
    <Tabs.Tab label='Worklogs' icon={Camera}>Gallery tab content</Tabs.Tab>
    <Tabs.Tab label='Config' icon={Gear}>Settings tab content</Tabs.Tab>
</Tabs>