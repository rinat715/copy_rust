<script lang="ts">
    import { Container, Group } from '@svelteuidev/core';
    import Date from './date.svelte';
    let startDate: string;
    let endDate: string;
    
</script>

<Container override={{bc: 'AliceBlue'}}>
    <Group position="center">
        <Date label="Start Date" bind:value={startDate}/>
        <Date label="End Date" bind:value={endDate}/>
    </Group>
</Container>
