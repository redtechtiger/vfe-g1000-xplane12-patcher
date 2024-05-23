<script lang="ts">
  enum Status {
    PATH_VALID,
    PATH_INVALID,
    WORKING_PATCHING,
    WORKING_RESTORING,
    ERROR_FAILED,
    ERROR_INVALID_VERSION,
    NOCHANGE,
    SUCCESS,
  };

  import { event } from "@tauri-apps/api";
  import { invoke } from "@tauri-apps/api/tauri";
  export let path = "";
  export let status = Status.PATH_INVALID;

  async function patch() {
    try {
      console.log("Getting installation directory...");
      let directory = await get_directory();

      console.log("Attempting patch...");
      status = Status.WORKING_PATCHING;
      let result = await invoke('patch', {path: path});
      console.log(result);

      await new Promise(f => setTimeout(f, 1000));

      switch(result) {
        case "Success": {
          status = Status.SUCCESS;
          break;
        }
        case "NoChange": {
          status = Status.NOCHANGE;
          break;
        }
        case "InvalidVersion": {
          status = Status.ERROR_INVALID_VERSION;
          break;
        }
        case "Failed": {
          status = Status.ERROR_FAILED;
          break;
        }
      }

    } catch {
      status = Status.ERROR_FAILED;
    }
  }
  
  async function restore() {
    try {
      console.log("Getting installation directory...");
      let directory = await get_directory();

      console.log("Attempting restore...");
      status = Status.WORKING_RESTORING;
      let result = await invoke('restore', {path: path});
      console.log(result);

      await new Promise(f => setTimeout(f, 1000));

      switch(result) {
        case "Success": {
          status = Status.SUCCESS;
          break;
        }
        case "NoChange": {
          status = Status.NOCHANGE;
          break;
        }
        case "InvalidVersion": {
          status = Status.ERROR_INVALID_VERSION;
          break;
        }
        case "Failed": {
          status = Status.ERROR_FAILED;
          break;
        }
      }

    }
    catch {
      status = Status.ERROR_FAILED;
    }
  }

  async function get_directory() {
    let result: boolean = await invoke('verify_path', {path: path})
    if(result===false) {
      status = Status.PATH_INVALID;
      throw Error("Directory doesn't appear to contain an installation.");
    }
    status = Status.PATH_VALID;
    return path;
  }

  event.listen<String>('tauri://file-drop', drop_event => {
    path = drop_event.payload + '';
    try {
    get_directory();
    } catch {}
  })
</script>

<main class="container">
  <div class="bottom-left">
    <p>© RedTechTiger 2024. No warranty provided.</p>
  </div>
  <div class="bottom-right">
    <p>v1.1</p>
  </div>
  <div class="top-section">
    <h1>XPlane 12 Patcher</h1>
    <p>
      This is a fully automated tool for patching the VFE G1000 XPlane control software for use with XPlane 12, that patches the main binary of the driver with an updated plugin TCP protocol handler. Please drag and drop the installation folder into the box below.
    </p>
  </div>
  <div class="middle-section">
    <ul class="status-section">
      {#if status===Status.PATH_INVALID}
      <p>
        ⚠️ Directory doesn't appear to contain an installation!
      </p>
      {/if}
      {#if status===Status.PATH_VALID}
      <p>
        ✔️ Directory seems to contain a valid installation!
      </p>
      {/if}
      {#if status===Status.WORKING_PATCHING}
      <div class="spinner"></div>
      <p>
        Installing patch...
      </p>
      {/if}
      {#if status==Status.WORKING_RESTORING}
      <div class="spinner"></div>
      <p>
        Removing patch...
      </p>
      {/if}
      {#if status==Status.ERROR_FAILED}
      <p>
        ❌ Failed to complete the requested operation!
      </p>
      {/if}
      {#if status===Status.SUCCESS}
      <p>
        ✅ Operation completed successfully!
      </p>
      {/if}
      {#if status==Status.NOCHANGE}
      <p>
        ❗The requested operation has already been ran! No changes have been made.
      </p>
      {/if}
      {#if status==Status.ERROR_INVALID_VERSION}
      <p>
        ❌ You are running an unsupported version!
      </p>
      {/if}
    </ul>    

    <div class="drop">
    {#if path!==""}
    <p>
      {path}
    </p>
    {/if}
    </div>
    <div class="button-group">
      <button on:click="{patch}">
        Install patch
      </button>
      <button on:click="{restore}">
        Remove patch
      </button>
    </div>
  </div>

</main>

<style>
  :root {
	  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  }

  p {
    text-wrap: pretty;
    /* width: 75%; */
    text-align: center;
    align-self: center;
    /* margin-left: 50%; /*
    /* transform: translate(-50%, 0%); */
  }
  
  button {
    border-style: solid;
    border-color: #ffffff;
    width: 150px;
    height: 40px;
    border-radius: 8px;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  }

  button:hover {
    border-color: #000000;
    transition-duration: 500ms;
    transition-timing-function: ease-in-out;
  }

  .status-section {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 0px;
  }

  .button-group {
    margin-top: 10px;
  }

  .drop {
    background-color: #fafafa;
    border-radius: 20px;
    border-width: 2px;
    border-style: dotted;
    border-color: #aaaaaa;
    width: 600px;
    height: 80px;
    margin: 0 auto;
    align-content: center;
  }

  .top-section {
    margin-top: 5%;
    margin-left: 50%;
    width: 75%;
    transform: translate(-50%, 0%);
    text-align: center;
  }

  .middle-section {
    text-align: center;
    margin-top: 50px;
  }

  .bottom-left {
    position: fixed;
    left: 10px;
    bottom: -10px;
  }

  .bottom-left p {
    font-size: 14px;
  }

  .bottom-right {
    position: fixed;
    right: 10px;
    bottom: -10px;
  }

  .bottom-right p {
    font-size: 14px;
  }

  .spinner {
    height: 20px;
    width: 20px;
    color: rgba(0, 0, 0, 0);
    position: relative;
    display: inline-block;
    border: 2px solid;
    margin: 10px;
    border-radius: 50%;
    border-right-color: #00AAff;
    animation: rotate 0.5s linear infinite;
  }

  @keyframes rotate {
    0% {
      transform: rotate(0); }
    100% {
      transform: rotate(360deg); }
  }


</style>
