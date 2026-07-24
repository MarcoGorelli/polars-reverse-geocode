import subprocess
import sys
import textwrap

import polars as pl
from polars.testing import assert_frame_equal

from polars_reverse_geocode import (
    find_closest_country,
    find_closest_state,
    reverse_geocode,
)


def test_main() -> None:
    df = pl.DataFrame({"lat": [37.7749, 51.01, 52.5], "lon": [-122.4194, -3.9, -0.91]})
    result = df.with_columns(city=reverse_geocode("lat", "lon"))
    expected = pl.DataFrame(
        {
            "lat": [37.7749, 51.01, 52.5],
            "lon": [-122.4194, -3.9, -0.91],
            "city": ["San Francisco", "South Molton", "Market Harborough"],
        }
    )
    assert_frame_equal(result, expected)


def test_find_closest_state() -> None:
    df = pl.DataFrame({"lat": [37.7749, 51.01, 52.5], "lon": [-122.4194, -3.9, -0.91]})
    result = df.with_columns(city=find_closest_state("lat", "lon"))
    expected = pl.DataFrame(
        {
            "lat": [37.7749, 51.01, 52.5],
            "lon": [-122.4194, -3.9, -0.91],
            "city": ["California", "England", "England"],
        }
    )
    assert_frame_equal(result, expected)


def test_find_closest_country() -> None:
    df = pl.DataFrame({"lat": [37.7749, 51.01, 52.5], "lon": [-122.4194, -3.9, -0.91]})
    result = df.with_columns(city=find_closest_country("lat", "lon"))
    expected = pl.DataFrame(
        {
            "lat": [37.7749, 51.01, 52.5],
            "lon": [-122.4194, -3.9, -0.91],
            "city": ["US", "GB", "GB"],
        }
    )
    assert_frame_equal(result, expected)


def _run_timing_script(script: str) -> None:
    # Each timing scenario is run in a fresh subprocess so that the
    # in-process geocoder cache from other tests (or earlier calls in
    # this same test) can't contaminate the "first call" timing.
    result = subprocess.run(
        [sys.executable, "-c", textwrap.dedent(script)],
        capture_output=True,
        text=True,
        check=False,
    )
    assert result.returncode == 0, (
        f"timing script failed:\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
    )


def test_cache_mode_do_not_cache_is_slow_on_every_call() -> None:
    # With caching disabled, every single-row call pays the full fixed
    # cost of loading the geocoder data set, so repeated calls should
    # all be slow (>10ms).
    script = """
        import time
        import polars as pl
        from polars_reverse_geocode import find_closest_city

        df = pl.DataFrame({"lat": [37.7749], "lon": [-122.4194]})

        for _ in range(2):
            start = time.perf_counter()
            df.with_columns(
                city=find_closest_city("lat", "lon", cache_mode="do_not_cache")
            )
            elapsed_ms = (time.perf_counter() - start) * 1000
            assert elapsed_ms > 10, (
                f"expected do_not_cache call to take >10ms, took {elapsed_ms:.2f}ms"
            )
    """
    _run_timing_script(script)


def test_cache_mode_cache_forever_caches_after_first_call() -> None:
    # With caching enabled, the first call pays the fixed cost of
    # loading the geocoder data set (>10ms), but subsequent calls
    # reuse the cached, in-memory geocoder and should be fast (<100ms).
    script = """
        import time
        import polars as pl
        from polars_reverse_geocode import find_closest_city

        df = pl.DataFrame({"lat": [37.7749], "lon": [-122.4194]})

        start = time.perf_counter()
        df.with_columns(
            city=find_closest_city("lat", "lon", cache_mode="cache_forever")
        )
        first_call_ms = (time.perf_counter() - start) * 1000
        assert first_call_ms > 10, (
            f"expected first cache_forever call to take >10ms, took {first_call_ms:.2f}ms"
        )

        start = time.perf_counter()
        df.with_columns(
            city=find_closest_city("lat", "lon", cache_mode="cache_forever")
        )
        second_call_ms = (time.perf_counter() - start) * 1000
        assert second_call_ms < 100, (
            f"expected cached call to take <100ms, took {second_call_ms:.2f}ms"
        )
    """
    _run_timing_script(script)
