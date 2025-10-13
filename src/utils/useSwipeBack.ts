import { onMounted, onUnmounted, ref } from "vue";
import { useRouter } from "vue-router";

export function useSwipeBack() {
  const router = useRouter();

  const transform = ref("");
  const transition = ref("none");

  let startX = 0;
  let startY = 0;
  let startTime = 0;
  let isSwiping = false;

  const canGoBack = () => {
    return window.history.length > 1;
  };

  const handleTouchStart = (e: TouchEvent) => {
    if (document.body.dataset.disableSwipeBack) return;

    if (!canGoBack()) return;

    if (e.touches[0].clientX > 50) return;

    startX = e.touches[0].clientX;
    startY = e.touches[0].clientY;
    startTime = Date.now();
    isSwiping = true;
    transition.value = "none";
  };

  const handleTouchMove = (e: TouchEvent) => {
    if (!isSwiping) return;

    const currentX = e.touches[0].clientX;
    const currentY = e.touches[0].clientY;
    const diffX = currentX - startX;
    const diffY = Math.abs(currentY - startY);

    if (diffY > Math.abs(diffX)) {
      isSwiping = false;
      transform.value = "";
      return;
    }

    if (diffX > 0) {
      e.preventDefault();
      const progress = diffX / window.innerWidth;
      const damping = 1 - progress * 0.5;
      const translateX = diffX * damping;
      transform.value = `translateX(${translateX}px)`;
    }
  };

  const handleTouchEnd = (e: TouchEvent) => {
    if (!isSwiping) return;

    const endX = e.changedTouches[0].clientX;
    const diffX = endX - startX;
    const diffTime = Date.now() - startTime;

    const velocity = Math.abs(diffX) / diffTime;

    const distanceThreshold = window.innerWidth * 0.4;
    const velocityThreshold = 0.5;

    transition.value = "transform 0.3s cubic-bezier(0.4, 0, 0.2, 1)";

    const shouldGoBack = diffX > distanceThreshold || (diffX > 80 && velocity > velocityThreshold);

    if (shouldGoBack && canGoBack()) {
      transform.value = `translateX(${window.innerWidth}px)`;
      setTimeout(() => {
        router.back();
        setTimeout(() => {
          transform.value = "";
          transition.value = "none";
        }, 50);
      }, 300);
    } else {
      transform.value = "";
    }

    isSwiping = false;
  };

  onMounted(() => {
    document.addEventListener("touchstart", handleTouchStart, { passive: true });
    document.addEventListener("touchmove", handleTouchMove, { passive: false });
    document.addEventListener("touchend", handleTouchEnd, { passive: true });
  });

  onUnmounted(() => {
    document.removeEventListener("touchstart", handleTouchStart);
    document.removeEventListener("touchmove", handleTouchMove);
    document.removeEventListener("touchend", handleTouchEnd);
  });

  return {
    transform,
    transition,
  };
}
