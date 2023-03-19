<script>
    import { SystemDrive } from "$env/static/private";
    import { invoke } from "@tauri-apps/api/tauri";

    let path = "";

    let name = "";
    let greetMsg = "";
    let new_master_password = "";
    let new_password_register = true;

    let is_master_password_present = invoke("is_file_here", { path }).then(
        (message) => message + ""
    );

    async function greet() {
        greetMsg = await invoke("greet", { name });
    }

    async function register() {
        await invoke("register_master_password", { new_master_password });
    }
</script>

<div class="log_screen">
    <div class="input_pass">
        <input
            id="greet-input"
            placeholder="Enter a name..."
            bind:value={new_master_password}
            class="input_pass"
        />
    </div>
    <div class="button_part">
        <div class="button_pass">
            {#if new_password_register}
                <button on:click={register}>register</button>
            {/if}
        </div>
        <div class="logo">
            <img src="" alt="" />
        </div>
    </div>
</div>

<style>
    div.log_screen {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    div.input_pass {
        flex: 1;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    input.input_pass {
        box-sizing: border-box;
        position: absolute;
        width: 30vw;
        height: 28px;
        background: #ffffff;
        border-radius: 9px;
    }

    div.button_part {
        flex: 1;
        display: flex;
        flex-direction: row;
    }

    div.button_pass {
        flex: 1;
        padding: 10px;
    }

    div.button_pass > button {
        width: 10vw;
        height: 5vh;
    }
</style>
