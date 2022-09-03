import { Component, createSignal, For } from 'solid-js'
import Splitter from 'grapheme-splitter'

interface GameProps {
  gameId: string
}
type Color = 'white' | 'black'
type Cell = '⚪' | '⬜' | '⚫' | '⬛' | '🟦'
type Board = Cell[][]
type Game = {
  board: Board
  turn: Color
  just_take?: Position
}
type Position = [number, number]
type ParsedData = {
  turn: Color
  just_take?: Position
  board: string
}
const Game: Component<GameProps> = ({ gameId }) => {
  const wsUrl = `ws://${import.meta.env.VITE_SOCKET_URL}/game/${gameId}`
  const splitter = new Splitter()
  const socket = new WebSocket(wsUrl)
  const [game, setGame] = createSignal<Game>({ board: [], turn: 'white' })
  const [selected, setSelected] = createSignal<Position | null>(null)
  const [color, setColor] = createSignal<Color | null>(null)
  const skippable = () => game().just_take && game().turn === color()
  const handleMessage = (e: MessageEvent) => {
    switch (e.data) {
      case 'full': {
        alert('Game is full')
        return
      }
      case 'white':
      case 'black': {
        setColor(e.data)
        return
      }
    }
    const { board, turn, just_take } = JSON.parse(e.data) as ParsedData
    const newBoard: Board = [...Array(8)].map(() => Array(8).fill('⬜'))
    splitter.splitGraphemes(board).forEach((cell, i) => {
      const row = Math.floor(i / 8)
      const col = i % 8
      newBoard[row][col] = cell as Cell
    })
    if (color() === 'black') newBoard.reverse().forEach((row) => row.reverse())
    setGame({ board: newBoard, turn, just_take })
  }
  const getColor = (row: number, col: number): Color | 'space' => {
    switch (game().board[row][col]) {
      case '⚪':
      case '⬜':
        return 'white'
      case '⚫':
      case '⬛':
        return 'black'
      default:
        return 'space'
    }
  }
  const handleCellClick = (row: number, col: number) => {
    if (selected()) {
      const [selectedRow, selectedCol] = selected()!
      color() === 'white'
        ? socket.send(`${selectedRow},${selectedCol},${row},${col}`)
        : socket.send(
            `${7 - selectedRow},${7 - selectedCol},${7 - row},${7 - col}`
          )
      setSelected(null)
    } else if (getColor(row, col) === color()) setSelected([row, col])
  }
  const handleSkipClick = () => socket.send('skip')

  socket.onmessage = handleMessage
  socket.onclose = () => alert('Disconnected')
  if (!socket.OPEN) return <p class="text-center">Loading...</p>
  return (
    <>
      <div class="flex mb-4 items-center">
        <div
          class={`bg-green-500 w-4 h-4 mr-2 rounded-full ${
            game().turn === color() ? 'translate-y-40' : '-translate-y-40'
          } transition-all duration-300 ease-in`}
        ></div>
        <div class="grid grid-cols-8 aspect-square gap-1 w-fit">
          <For each={game().board}>
            {(row, i) => (
              <For each={row}>
                {(cell, j) => (
                  <div
                    class={`aspect-square w-10 text-center text-2xl align-middle leading-10 cursor-pointer ${
                      selected()?.[0] === i() && selected()?.[1] === j()
                        ? 'bg-slate-200'
                        : 'bg-slate-500 hover:bg-slate-400'
                    } transition-all duration-200 after:content-['${cell}']`}
                    onClick={() => handleCellClick(i(), j())}
                  ></div>
                )}
              </For>
            )}
          </For>
        </div>
      </div>
      <button
        onClick={handleSkipClick}
        disabled={!skippable()}
        class="px-4 py-2 rounded-full bg-slate-500 disabled:opacity-50 enabled:hover:bg-slate-400 transition-all duration-200"
      >
        End Turn
      </button>
    </>
  )
}

export default Game
