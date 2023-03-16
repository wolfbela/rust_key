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

<body>
    <div>
        <input
            id="greet-input"
            placeholder="Enter a name..."
            bind:value={new_master_password}
        />
        <button on:click={greet}>Greet</button>
        <button on:click={register}>register</button>
        <p>{greetMsg}</p>
    </div>
</body>

<style>
    body {
        width: 100vw;
        height: 100vh;
        display: flex;
    }

    body > div {
        flex: 1;
        justify-content: center;
        align-items: center;
    }
</style>
