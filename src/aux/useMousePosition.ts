import { useState, useEffect } from "react";

interface Position {
  x: number,
  y: number,
}

const useMousePosition = () => {
  const [mousePosition, setMousePosition] = useState<Position>({
    x: 0,
    y: 0,
  })

  const handleMouse = (event: MouseEvent) => {
    setMousePosition({ x: event.clientX, y: event.clientY });
  }

  const handleTouch = (event: TouchEvent) => {
    let touches = event.changedTouches || [];
    if (touches.length > 0) {
      let touch = touches[0];
      setMousePosition({ x: touch.clientX, y: touch.clientY });
    }
  }

  useEffect(() => {
    document.addEventListener("mousemove", handleMouse);
    document.addEventListener("touchstart", handleTouch);
    document.addEventListener("touchmove", handleTouch);
    document.addEventListener("touchend", handleTouch);
    document.addEventListener("touchcancel", handleTouch);

    return () => {
      document.removeEventListener("mousemove", handleMouse);
      document.removeEventListener("touchstart", handleTouch);
      document.removeEventListener("touchmove", handleTouch);
      document.removeEventListener("touchend", handleTouch);
      document.removeEventListener("touchcancel", handleTouch);
    };
  }, [handleMouse, handleTouch]);

  return mousePosition;
}

export { useMousePosition };
