<script>
    export let board;
    export let move;
</script>

<div class="board noselect">
    {#each board as row}
        <div class="row">
            {#each row as elem, i}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <span on:mousedown={() => {
                    console.log("clicked")
                    move(i);
                }} class="elem {elem == 1 ? "yellow" : (elem == 2 ? "red" : "blank")}">
                    <!-- {elem} -->
                </span>
            {/each}
        </div>
    {/each}
</div>

<style>
    span{
        font-size: xx-large;
        display: inline-block;
        margin: 0.3em;
        cursor: pointer;
        width: 0.6em;
        aspect-ratio: 1;
        text-align: center;
        border-radius: 1em;
    }
    .yellow{
        background-color: yellow;
    }
    .red{
        background-color: red;
    }
    .blank{
        background-color: white;
    }
</style>