import { useEffect, useRef, useState } from "react";
import { cn } from "@/lib/utils";

interface MarqueeProps {
  className?: string;
  reverse?: boolean;
  pauseOnHover?: boolean;
  children?: React.ReactNode;
  vertical?: boolean;
  repeat?: number;
  [key: string]: any;
}

export default function Marquee({
  className,
  reverse,
  pauseOnHover = false,
  children,
  vertical = false,
  repeat = 1,
  ...props
}: MarqueeProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [duration, setDuration] = useState(20); // 默认动画持续时间

  useEffect(() => {
    // 动态计算卡片总宽度
    if (containerRef.current) {
      const containerWidth = containerRef.current.scrollWidth; // 所有卡片的总宽度
      const viewWidth = containerRef.current.clientWidth; // 视口宽度

      // 动态调整动画时长 (根据总宽度和视口宽度)
      const calculatedDuration = (containerWidth / viewWidth) * 20; // 自定义公式，视项目需求调整
      setDuration(calculatedDuration);
    }
  }, [children]);

  return (
    <div
      {...props}
      ref={containerRef} // 绑定 ref，用于获取容器宽度
      className={cn(
        "group flex overflow-hidden p-2 w-full [--gap:1rem] [gap:var(--gap)]",
        {
          "flex-row": !vertical,
          "flex-col": vertical,
        },
        className,
      )}
      style={{ "--duration": `${duration}s` } as React.CSSProperties} // 动态设置动画时长
    >
      {Array(repeat)
        .fill(0)
        .map((_, i) => (
          <div
            key={i}
            className={cn("flex flex-nowrap w-auto shrink-0 [gap:var(--gap)]", {
              "animate-marquee flex-row": !vertical,
              "animate-marquee-vertical flex-col": vertical,
              "group-hover:[animation-play-state:paused]": pauseOnHover,
              "[animation-direction:reverse]": reverse,
            })}
          >
            {children}
          </div>
        ))}
    </div>
  );
}
