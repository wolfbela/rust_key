<script>
    import { invoke } from "@tauri-apps/api/tauri";

    let name = "";
    let greetMsg = "";
    let new_master_password = "";

    async function greet() {
        greetMsg = await invoke("greet", { name });
    }

    async function register() {
        await invoke("register_master_password", { new_master_password });
    }
</script>

<div>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>Greet</button>
    <button on:click={register}>register</button>
    <p>{greetMsg}</p>
</div>

<style>
    body {
        background-color: #ffb87e;
    }
</style>
