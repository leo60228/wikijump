<!--
  @component Generic "toggle" component.
-->
<script lang="ts">
  import { keyHandle } from "@wikijump/dom"

  /** Type of "toggle" to use. Only has a visual effect. */
  export let type: "slider" | "checkbox" = "slider"

  /** State of the toggle. */
  export let toggled = false

  /** Size of the toggle. */
  export let size = "1em"
</script>

<label class="toggleinput" class:is-toggled={toggled}>
  <input
    class="toggleinput-input"
    type="checkbox"
    bind:checked={toggled}
    use:keyHandle={{
      key: "Enter",
      do: () => void (toggled = !toggled)
    }}
    {...$$restProps}
  />
  <span class="toggleinput-wrapper" role="presentation">
    {#if $$slots.before}
      <span class="toggleinput-slot-before">
        <slot name="before" />
      </span>
    {/if}

    {#if type === "slider"}
      <svg class="toggleinput-sprite is-slider" height={size} viewBox="0 0 128 64">
        <rect class="toggleinput-track" x="2" y="2" width="124" height="60" rx="30" />
        <rect x="16" y="16" width="96" height="32" rx="16" fill="#0002" />
        <circle class="toggleinput-handle" cy="32" cx="32" r="26" />
      </svg>
    {:else if type === "checkbox"}
      <svg class="toggleinput-sprite is-checkbox" height={size} viewBox="0 0 64 64">
        <rect class="toggleinput-box" x="2" y="2" width="60" height="60" rx="8" />
        <path class="toggleinput-checkmark" d="m13.25 32 12.5 12.5 25 -25" />
      </svg>
    {/if}

    <span class="toggleinput-slot-after"><slot /></span>
  </span>
</label>

<style lang="scss">
  @import "../../../resources/css/abstracts";

  .toggleinput {
    position: relative;
    cursor: pointer;

    @include hover {
      .toggleinput-wrapper > span {
        color: var(--col-hint);
      }

      &:not(.is-toggled) {
        .toggleinput-checkmark {
          opacity: 0.25;
          stroke: var(--col-text);
        }
      }
    }

    &.is-toggled {
      .toggleinput-track {
        fill: var(--col-hint);
      }

      .toggleinput-handle {
        cx: 96px;
      }

      .toggleinput-box {
        fill: var(--col-hint);
        stroke: #222;
      }

      .toggleinput-checkmark {
        opacity: 1;
      }
    }
  }

  .toggleinput-track {
    fill: #aaa;

    @include tolerates-motion {
      transition: fill 150ms;
    }
  }

  .toggleinput-handle {
    fill: #fff;
    cx: 32px;

    @include tolerates-motion {
      transition: cx 150ms, fill 150ms;
    }
  }

  .toggleinput-box {
    fill: none;
    stroke: var(--col-con-border);
    stroke-width: 4;

    @include tolerates-motion {
      transition: fill 75ms, stroke 75ms;
    }
  }

  .toggleinput-checkmark {
    fill: none;
    stroke: #fff;
    stroke-width: 8;
    stroke-linecap: round;
    stroke-linejoin: round;
    opacity: 0;

    @include tolerates-motion {
      transition: opacity 75ms;
    }
  }

  .toggleinput-wrapper {
    display: flex;
    align-items: center;

    > span {
      font-size: 0.875em;
      user-select: none;

      @include tolerates-motion {
        transition: color 150ms;
      }
    }

    .toggleinput-slot-after {
      margin-left: 0.5em;
    }
  }

  .toggleinput-input {
    position: absolute;
    width: 0;
    height: 0;
    top: 0;
    left: 0;
    opacity: 0;

    &:focus-visible ~ .toggleinput-wrapper {
      outline: 5px auto Highlight;
      outline: 5px auto -webkit-focus-ring-color;

      > span {
        color: var(--col-hint);
      }
    }
  }
</style>
