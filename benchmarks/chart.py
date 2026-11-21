"""Draw responsive benchmark cards and animate the data marks in SVG."""

from html import escape
from io import StringIO
import math
from pathlib import Path
import re

from matplotlib import rc_context
from matplotlib.backends.backend_svg import FigureCanvasSVG
from matplotlib.figure import Figure
from matplotlib.lines import Line2D
from matplotlib.patches import FancyBboxPatch
from matplotlib.ticker import NullLocator

LIGHT = {
    "canvas": "#ffffff", "card": "#f6f8fa", "border": "#d0d7de",
    "text": "#24292f", "secondary": "#57606a", "grid": "#e1e6ec",
    "cpp": "#0969da", "rust": "#a04d00",
}
DARK = {
    "canvas": "#0d1117", "card": "#161b22", "border": "#30363d",
    "text": "#e6edf3", "secondary": "#9da7b3", "grid": "#30363d",
    "cpp": "#58a6ff", "rust": "#ffa657",
}


def format_ms(value):
    if value < 0.1:
        return f"{value:.4f}"
    if value < 10:
        return f"{value:.3f}"
    return f"{value:.2f}"


def ratio_scale(tiers):
    ratios = [case["rust_over_cpp"] for tier in tiers for case in tier["cases"]]
    first = math.floor(math.log2(min(0.5, *(ratio["p25"] for ratio in ratios))))
    last = math.ceil(math.log2(max(2.0, *(ratio["p75"] for ratio in ratios))))
    step = max(1, math.ceil((last - first) / 6))
    ticks = [2.0 ** power for power in sorted({0} | set(range(first, last + 1, step)))]
    return 2.0 ** first, 2.0 ** last, ticks


def motion_style(case_count):
    delays = "\n".join(
        f'    [id^="benchmark-{i}-"] {{ animation-delay: {i * 35}ms; }}'
        for i in range(case_count)
    )
    return f"""<style type="text/css">
  @keyframes benchmark-enter {{
    from {{ opacity: 0; transform: translateY(4px); }}
    to {{ opacity: 1; transform: translateY(0); }}
  }}
  @media (prefers-reduced-motion: no-preference) {{
    [id^="benchmark-"] {{ animation: benchmark-enter 450ms ease-out both; }}
{delays}
  }}
</style>"""


def render_chart(report, theme, compact, output, animate=True):
    tiers = report["tiers"]
    width = 5.1 if compact else 10
    panel_height = 4.6 if compact else 2.94
    height = 1.38 + len(tiers) * (panel_height + 0.18)
    figure = Figure(figsize=(width, height), facecolor=theme["canvas"])
    canvas = FigureCanvasSVG(figure)

    def text(x, y, label, *, color="text", **kwargs):
        return figure.text(x / width, y / height, label, color=theme[color], va="center", **kwargs)

    text(0.32, height - 0.30, "C++23 vs Rust", fontsize=21 if compact else 25, weight="bold")
    text(0.32, height - 0.60, f"{report['machine']}  ·  {report['samples']} paired samples",
         fontsize=9, color="secondary")
    revision = "Local preview" if report["dirty"] else f"Revision {report['commit'][:7]}"
    text(0.32, height - 0.81, f"{revision}  ·  Optimized builds", fontsize=9, color="secondary")

    lower, upper, ticks = ratio_scale(tiers)
    index = 0
    for panel, tier in enumerate(tiers):
        top = height - 1.03 - panel * (panel_height + 0.18)
        bottom = top - panel_height
        figure.add_artist(FancyBboxPatch(
            (0.15 / width, bottom / height),
            (width - 0.30) / width, panel_height / height,
            boxstyle="round,pad=0,rounding_size=0.012",
            transform=figure.transFigure, facecolor=theme["card"],
            edgecolor=theme["border"], linewidth=0.8, zorder=-1,
        ))
        text(0.34, top - 0.25, f"{tier['name']} inputs", fontsize=15, weight="bold")

        if compact:
            plot_left, plot_width = 0.43, width - 0.86
            cpp_x, rust_x = width - 1.20, width - 0.37
        else:
            plot_left, plot_width = 5.65, width - 6.05
            cpp_x, rust_x = 3.62, 4.82

        plot_bottom = bottom + 0.37
        plot_height = panel_height - 0.95
        header_y = top - 0.45
        text(cpp_x, header_y, "C++23 ms", fontsize=8, ha="right", color="cpp", weight="bold")
        text(rust_x, header_y, "Rust ms", fontsize=8, ha="right", color="rust", weight="bold")

        cases = tier["cases"]
        axis = figure.add_axes(
            [plot_left / width, plot_bottom / height, plot_width / width, plot_height / height],
            facecolor="none",
        )
        axis.set_xscale("log", base=2)
        axis.set_xlim(lower, upper)
        axis.set_ylim(-0.5, len(cases) - 0.5)
        axis.set_xticks(ticks, [f"{tick:g}×" for tick in ticks])
        axis.xaxis.set_minor_locator(NullLocator())
        axis.set_yticks([])
        axis.tick_params(axis="x", colors=theme["secondary"], labelsize=8, length=0, pad=3)
        axis.grid(axis="x", color=theme["grid"], linewidth=0.7)
        axis.set_axisbelow(True)
        axis.axvline(1, color=theme["secondary"], linewidth=1, linestyle="--", alpha=0.65)
        for spine in axis.spines.values():
            spine.set_visible(False)

        if not compact:
            text(plot_left, header_y, "Rust faster", fontsize=8, color="rust")
            text(plot_left + plot_width, header_y, "C++23 faster",
                 fontsize=8, color="cpp", ha="right")

        for row, case in enumerate(cases):
            y = len(cases) - row - 1
            row_y = plot_bottom + plot_height * (y + 0.5) / len(cases)
            label_y = row_y + (0.09 if compact else 0)
            point_y = y - (0.19 if compact else 0)
            if row:
                line_y = plot_bottom + plot_height * (y + 1) / len(cases)
                figure.add_artist(Line2D(
                    [0.33 / width, 1 - 0.33 / width], [line_y / height] * 2,
                    transform=figure.transFigure, color=theme["grid"], linewidth=0.7,
                    zorder=1,
                ))
            text(0.34, label_y, case["label"], fontsize=10, weight="bold")
            for language, x in (("cpp", cpp_x), ("rust", rust_x)):
                milliseconds = format_ms(case[f"{language}_ms"]["median"])
                text(x, label_y, milliseconds, fontsize=9, color=language,
                     ha="right", family="monospace").set_gid(f"benchmark-{index}-{language}")
            ratio = case["rust_over_cpp"]
            median = ratio["median"]
            color = theme["rust"] if median < 1 else theme["cpp"]
            whisker = axis.errorbar(
                median, point_y,
                xerr=[[median - ratio["p25"]], [ratio["p75"] - median]],
                fmt="o", color=color, markersize=5, capsize=3, linewidth=1.6, zorder=3,
            )
            marker, caps, bars = whisker.lines
            for part, artist in enumerate([marker, *caps, *bars]):
                artist.set_gid(f"benchmark-{index}-ratio-{part}")
            index += 1

    legend = "Rust ÷ C++23 runtime · dot = median · whisker = middle 50%"
    text(0.32, 0.29, legend, fontsize=8 if compact else 9, color="secondary")
    text(0.32, 0.12, "Below 1× favors Rust. Above 1× favors C++23.", fontsize=8, color="secondary")
    raw = StringIO()
    canvas.print_svg(raw, metadata={"Date": None})
    title = "<title>C++23 and Rust benchmark comparison</title>"
    description = (
        "<desc>Median runtimes and middle fifty percent of paired ratios for "
        f"{escape(', '.join(tier['name'].lower() for tier in tiers))} inputs. "
        "An accessible results table accompanies the chart.</desc>"
    )
    extras = "\n".join([title, description, motion_style(index) if animate else ""])
    svg = re.sub(r"(<svg\b[^>]*>)", lambda match: match[0] + "\n" + extras, raw.getvalue(), count=1)
    output.write_text(svg, encoding="utf-8")


def render_charts(report, output: Path):
    with rc_context({"svg.fonttype": "none", "svg.hashsalt": "benchmarks"}):
        for suffix, theme in (("", LIGHT), ("-dark", DARK)):
            render_chart(report, theme, False, output / f"comparison{suffix}.svg")
            render_chart(report, theme, True, output / f"comparison-mobile{suffix}.svg")
        render_chart(report, LIGHT, False, output / "comparison-static.svg", animate=False)
