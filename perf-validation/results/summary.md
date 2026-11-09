# Local verification results

Status: **passed**.

Host: `Windows-10-10.0.26200-SP0`. CPU: `Intel64 Family 6 Model 183 Stepping 1, GenuineIntel`.

Timings describe this host and these synthetic fixtures. They do not establish universal speedups.

## Executed checks

| Check | Passed tests | Result |
|---|---:|---|
| toolchain | 0 | passed |
| format | 0 | passed |
| clippy | 0 | passed |
| tests-debug | 46 | passed |
| tests-release | 46 | passed |
| benchmark-list | 0 | passed |
| benchmark-smoke | 0 | passed |
| benchmark-measurements | 0 | passed |
| build-release | 0 | passed |
| probe-release | 0 | passed |
| build-release-lto | 0 | passed |
| probe-release-lto | 0 | passed |
| build-release-codegen | 0 | passed |
| probe-release-codegen | 0 | passed |
| build-release-tuned | 0 | passed |
| probe-release-tuned | 0 | passed |
| nightly-toolchain | 0 | passed |
| nightly-collect-into | 0 | passed |

Nightly collect_into: passed.

## Criterion measurements

Means and their 95% confidence intervals are in nanoseconds per iteration. Different sizes or scopes are not A/B pairs.

### perf-ahash

Measured cases: 36.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| ahash/integers/repeated/32 | 1565.898 | 1246.097–1926.646 |
| ahash/integers/repeated/4096 | 331675.940 | 280550.170–382549.202 |
| ahash/integers/unique/32 | 2425.976 | 2280.938–2622.893 |
| ahash/integers/unique/4096 | 369312.549 | 333654.263–411781.329 |
| ahash/strings/repeated/width128/32 | 1260.312 | 1254.854–1266.376 |
| ahash/strings/repeated/width128/4096 | 530868.481 | 443934.702–632295.934 |
| ahash/strings/repeated/width8/32 | 963.047 | 955.310–972.501 |
| ahash/strings/repeated/width8/4096 | 316008.266 | 261198.188–371990.613 |
| ahash/strings/unique/width128/32 | 3667.651 | 2970.790–4448.139 |
| ahash/strings/unique/width128/4096 | 476013.214 | 398002.232–558597.504 |
| ahash/strings/unique/width8/32 | 2001.387 | 1636.493–2421.808 |
| ahash/strings/unique/width8/4096 | 351738.726 | 280013.897–427057.001 |
| fx/integers/repeated/32 | 2762.955 | 2480.478–3054.795 |
| fx/integers/repeated/4096 | 256463.672 | 241377.104–278464.302 |
| fx/integers/unique/32 | 3098.728 | 2801.171–3396.516 |
| fx/integers/unique/4096 | 175255.575 | 155366.634–197298.398 |
| fx/strings/repeated/width128/32 | 1125.141 | 1118.393–1132.396 |
| fx/strings/repeated/width128/4096 | 371202.137 | 325201.448–416692.019 |
| fx/strings/repeated/width8/32 | 888.884 | 883.517–895.224 |
| fx/strings/repeated/width8/4096 | 387775.962 | 343034.499–439339.767 |
| fx/strings/unique/width128/32 | 4622.095 | 4059.459–5203.875 |
| fx/strings/unique/width128/4096 | 415062.705 | 370413.447–463510.515 |
| fx/strings/unique/width8/32 | 4290.950 | 3828.349–4798.116 |
| fx/strings/unique/width8/4096 | 454281.721 | 416119.462–491554.256 |
| std/integers/repeated/32 | 1036.268 | 1032.607–1040.365 |
| std/integers/repeated/4096 | 374703.143 | 336396.910–414773.065 |
| std/integers/unique/32 | 3994.060 | 3570.984–4425.065 |
| std/integers/unique/4096 | 436231.411 | 397882.994–476692.119 |
| std/strings/repeated/width128/32 | 1752.106 | 1743.793–1762.706 |
| std/strings/repeated/width128/4096 | 698303.937 | 615764.355–786804.664 |
| std/strings/repeated/width8/32 | 2386.903 | 2026.423–2742.626 |
| std/strings/repeated/width8/4096 | 439939.705 | 384266.698–494064.662 |
| std/strings/unique/width128/32 | 4688.202 | 4193.398–5240.310 |
| std/strings/unique/width128/4096 | 697642.042 | 618223.801–788039.823 |
| std/strings/unique/width8/32 | 4545.712 | 3771.054–5459.481 |
| std/strings/unique/width8/4096 | 527840.242 | 471886.900–583420.079 |

### perf-black-box-bench

Measured cases: 3.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| discarded_constant_input | 1.331 | 1.328–1.333 |
| opaque_input_and_output | 1.490 | 1.479–1.500 |
| opaque_input_returned_output | 1.366 | 1.363–1.370 |

### perf-chain-avoid

Measured cases: 4.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| chain/32 | 58.631 | 52.014–65.660 |
| chain/4096 | 2055.828 | 2047.350–2065.954 |
| separate/32 | 8.660 | 8.136–9.511 |
| separate/4096 | 830.765 | 808.162–869.493 |

### perf-collect-into

Measured cases: 4.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| fresh/32 | 5959.907 | 4938.418–7272.805 |
| fresh/4096 | 123211.681 | 110453.632–136181.372 |
| stable-extend-reused/32 | 1179.668 | 1019.702–1349.964 |
| stable-extend-reused/4096 | 76708.395 | 64767.982–90802.565 |

### perf-collect-once

Measured cases: 8.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| once/mixed-32 | 804.421 | 640.537–974.092 |
| once/mixed-4096 | 97045.523 | 84454.378–112764.301 |
| once/none-32 | 911.385 | 806.152–1056.813 |
| once/none-4096 | 79181.246 | 66585.370–92792.455 |
| staged/mixed-32 | 775.261 | 747.084–804.446 |
| staged/mixed-4096 | 111061.424 | 98602.275–124623.442 |
| staged/none-32 | 1331.484 | 1094.043–1597.593 |
| staged/none-4096 | 578030.719 | 284141.551–927941.454 |

### perf-drain-reuse

Measured cases: 4.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| fresh/32 | 196.680 | 181.821–212.915 |
| fresh/4096 | 49891.717 | 47748.184–51778.178 |
| reused/32 | 106.994 | 95.612–119.731 |
| reused/4096 | 42689.890 | 35362.001–50679.761 |

### perf-entry-api

Measured cases: 8.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| entry-repeated/32 | 5123.922 | 4625.336–5629.198 |
| entry-repeated/4096 | 525010.881 | 431708.140–628195.988 |
| entry-unique/32 | 4274.046 | 4094.892–4510.287 |
| entry-unique/4096 | 777144.758 | 701240.489–856037.898 |
| reference-repeated/32 | 1686.298 | 1679.569–1693.793 |
| reference-repeated/4096 | 425600.757 | 382521.840–473664.508 |
| reference-unique/32 | 4984.855 | 4621.413–5364.465 |
| reference-unique/4096 | 659068.377 | 558888.401–773401.356 |

### perf-extend-batch

Measured cases: 4.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| extend/32 | 113.617 | 104.325–122.772 |
| extend/4096 | 806.857 | 696.625–905.911 |
| push/32 | 352.083 | 327.482–378.585 |
| push/4096 | 6895.707 | 6273.452–7548.869 |

### perf-io-buffering

Measured cases: 12.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| read/buffered-1024/32 | 31191.157 | 29683.618–32934.628 |
| read/buffered-1024/512 | 23553.951 | 22183.430–25164.815 |
| read/buffered-8192/32 | 34835.904 | 31707.002–38722.688 |
| read/buffered-8192/512 | 12968.184 | 12706.637–13311.080 |
| read/unbuffered/32 | 1113957.967 | 1010049.021–1219528.486 |
| read/unbuffered/512 | 17440053.333 | 14671052.375–20339877.417 |
| write/buffered-1024/32 | 50318.250 | 48754.296–52102.959 |
| write/buffered-1024/512 | 54030.318 | 50592.983–58680.471 |
| write/buffered-8192/32 | 48304.560 | 46600.692–50080.050 |
| write/buffered-8192/512 | 31668.238 | 30051.875–33658.991 |
| write/unbuffered/32 | 697801.602 | 669404.009–730972.181 |
| write/unbuffered/512 | 2007091.671 | 1912634.679–2123747.123 |

### perf-iter-lazy

Measured cases: 12.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| eager/absent-32 | 17.019 | 14.424–19.726 |
| eager/absent-4096 | 1944.174 | 1721.929–2184.208 |
| eager/early-32 | 64.157 | 51.038–78.215 |
| eager/early-4096 | 1823.678 | 1667.452–1982.572 |
| eager/late-32 | 79.344 | 72.312–88.660 |
| eager/late-4096 | 1784.752 | 1581.164–1997.944 |
| lazy/absent-32 | 27.725 | 24.625–30.872 |
| lazy/absent-4096 | 1160.685 | 955.088–1421.748 |
| lazy/early-32 | 2.606 | 2.415–2.810 |
| lazy/early-4096 | 2.223 | 1.990–2.480 |
| lazy/late-32 | 20.043 | 16.414–23.866 |
| lazy/late-4096 | 2441.556 | 2034.487–2849.871 |

### perf-iter-over-index

Measured cases: 4.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| indexed/32 | 6.285 | 6.152–6.458 |
| indexed/4096 | 1993.235 | 1704.118–2347.534 |
| iterator/32 | 6.166 | 6.106–6.257 |
| iterator/4096 | 1775.200 | 1708.119–1844.066 |

### perf-profile-first

Measured cases: 4.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| kernel_only/32 | 377.852 | 376.729–379.167 |
| kernel_only/4096 | 50396.344 | 50133.762–50696.556 |
| whole_pipeline/32 | 406.576 | 405.271–407.993 |
| whole_pipeline/4096 | 50740.208 | 50582.276–50937.836 |

### perf-release-profile

Measured cases: 1.

| Case | Mean (ns) | 95% interval (ns) |
|---|---:|---|
| checksum | 50020.610 | 49908.220–50136.878 |

## Production profile artifacts

Kernel medians are warm in-process measurements; build caches differ, so build durations are not compile-cost comparisons.

| Profile | Binary bytes | Kernel median (ns) | Drop on panic |
|---|---:|---:|---|
| release | 156672 | 1592300 | True |
| release-lto | 158208 | 1587300 | True |
| release-codegen | 156672 | 1629600 | True |
| release-tuned | 138752 | 1597800 | False |

Source mappings and counterexamples: [findings](README.md). Raw run record: [latest.json](latest.json).
