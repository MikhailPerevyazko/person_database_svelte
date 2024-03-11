<script setup lang="ts">
import {persons} from '../scripts/personsStorage'

</script>

<div class="m-4 justify-center border border-emerald-400">
    <ul>
        {#if $persons}
        {#each $persons.persons as person }
		<li>{person.id} - {person.name}</li>
	    {/each}
        {/if}
        
    </ul>
</div>
  
<style></style>