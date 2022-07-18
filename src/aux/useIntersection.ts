import { useState, useEffect, MutableRefObject } from 'react'

const useIntersection = (elementRef: MutableRefObject<HTMLAnchorElement | null | undefined>, rootMargin: string) => {
  const [isVisible, setState] = useState(false)

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        setState(entry.isIntersecting)
      }, { rootMargin }
    )

    elementRef.current && observer.observe(elementRef.current)

    return () => {
      elementRef.current && observer.unobserve(elementRef.current)
    }
  }, [elementRef])

  return isVisible
}

export { useIntersection }
