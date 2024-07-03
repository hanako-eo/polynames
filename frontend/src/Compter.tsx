import { createReactor } from "yeap/app"
import { Component } from "yeap/components"

export const Compter: Component<{ start?: number, step?: number }> = ({
  start = 0,
  step = 1,
}) => {
  const compter = createReactor(start)

  return (
    <p>
      {compter}
      <br />
      <button class="btn" onClick={() => compter((n) => n + step)}>
        increment
      </button>
    </p>
  )
}
