"""绘制单电子的三维轨迹与动能随时间变化。"""

from pathlib import Path

import matplotlib.pyplot as plt
import pandas as pd

PROJECT_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = PROJECT_ROOT / "results" / "trajectories" / "electron.csv"
FIGURE_DIR = PROJECT_ROOT / "results" / "figures"


def main() -> None:
    if not CSV_PATH.exists():
        raise FileNotFoundError(
            f"尚未生成 {CSV_PATH}\n请在项目根目录运行：cargo run --release"
        )

    FIGURE_DIR.mkdir(parents=True, exist_ok=True)
    df = pd.read_csv(CSV_PATH)

    # 位移 m -> mm，时间 s -> ns。
    x, y, z = [df[column] * 1e3 for column in ("x_m", "y_m", "z_m")]
    time_ns = df["time_s"] * 1e9

    fig = plt.figure(figsize=(9, 7))
    ax = fig.add_subplot(111, projection="3d")
    ax.plot(x, y, z, linewidth=0.8)
    ax.set_xlabel("X (mm)")
    ax.set_ylabel("Y (mm)")
    ax.set_zlabel("Z (mm)")
    ax.set_title("Electron trajectory")
    fig.tight_layout()
    file_trajectory = FIGURE_DIR / "electron_trajectory.png"
    fig.savefig(file_trajectory, dpi=300, bbox_inches="tight")
    print(f"图像已保存：{file_trajectory}")

    fig, ax = plt.subplots(figsize=(10, 5))
    ax.plot(time_ns, df["energy_eV"], linewidth=1.5)
    ax.set_xlabel("Time (ns)")
    ax.set_ylabel("Kinetic energy (eV)")
    ax.set_title("Electron kinetic energy")
    ax.grid(True, alpha=0.3)
    fig.tight_layout()
    file_energy = FIGURE_DIR / "electron_energy.png"
    fig.savefig(file_energy, dpi=300, bbox_inches="tight")
    print(f"图像已保存：{file_energy}")
    plt.show()


if __name__ == "__main__":
    main()
