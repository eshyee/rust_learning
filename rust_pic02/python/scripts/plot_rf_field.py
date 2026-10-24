"""绘制 Rust 测试生成的 RF 电场随时间变化曲线。"""

from pathlib import Path

import matplotlib.pyplot as plt
import pandas as pd

# 用脚本所在位置推导根目录，不依赖终端当前路径。
PROJECT_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = PROJECT_ROOT / "results" / "fields" / "rf_field_test.csv"
FIGURE_DIR = PROJECT_ROOT / "results" / "figures"


def main() -> None:
    if not CSV_PATH.exists():
        raise FileNotFoundError(
            f"尚未生成 {CSV_PATH}\n"
            "请在项目根目录运行：cargo test test_field_to_csv -- --nocapture"
        )

    FIGURE_DIR.mkdir(parents=True, exist_ok=True)
    df = pd.read_csv(CSV_PATH)

    fig, ax = plt.subplots(figsize=(12, 5))
    ax.plot(df["time_ns"], df["Ex_V_m"], label="Ex (13.56 MHz)", linewidth=1.5)
    ax.set_xlabel("Time (ns)")
    ax.set_ylabel("Electric field (V/m)")
    ax.set_title("RF electric field vs time")
    ax.grid(True, alpha=0.3)
    ax.legend()
    fig.tight_layout()

    figure_path = FIGURE_DIR / "rf_field.png"
    fig.savefig(figure_path, dpi=300, bbox_inches="tight")
    print(f"图像已保存：{figure_path}")
    plt.show()


if __name__ == "__main__":
    main()
