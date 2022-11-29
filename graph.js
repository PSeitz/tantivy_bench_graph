function loadState(callback) {
    let params = new URLSearchParams(window.location.search.slice(1))
    let state = {}
    for (let param of params) {
        let key = param[0]
        let value = param[1]
        state[key] = value
    }
    // Handle renaming of `run_name` to `scenario`
    if (state.run_name && !state.scenario) {
        state.scenario = state.run_name
        state.run_name = undefined
    }
    if (state.start) {
        document.getElementById('start-bound').value = state.start
    }
    if (state.end) {
        document.getElementById('end-bound').value = state.end
    }

    callback(state)
}

function tooltipPlugin({
    onclick,
    shiftX = 10,
    shiftY = 10,
    commit_hashes,
    commit_hash_to_msg,
}) {
    let tooltipLeftOffset = 0
    let tooltipTopOffset = 0

    const tooltip = document.createElement('div')
    tooltip.className = 'u-tooltip'

    let seriesIdx = null
    let dataIdx = null

    const fmtDate = uPlot.fmtDate('{D}/{M}/{YYYY} {h}:{mm}:{ss} ')

    let tooltipVisible = false

    function showTooltip() {
        if (!tooltipVisible) {
            tooltip.style.display = 'block'
            over.style.cursor = 'pointer'
            tooltipVisible = true
        }
    }

    function hideTooltip() {
        if (tooltipVisible) {
            tooltip.style.display = 'none'
            over.style.cursor = null
            tooltipVisible = false
        }
    }

    function setTooltip(u) {
        showTooltip()

        let top = u.valToPos(u.data[seriesIdx][dataIdx], 'y')
        let lft = u.valToPos(u.data[0][dataIdx], 'x')

        tooltip.style.top = tooltipTopOffset + top + shiftX + 'px'

        //tooltip.style.borderColor = isInterpolated(dataIdx) ? interpolatedColor : seriesColors[seriesIdx - 1];
        let pctSinceStart = (
            ((u.data[seriesIdx][dataIdx] - u.data[seriesIdx][0]) /
                u.data[seriesIdx][0]) *
            100
        ).toFixed(2)
        let commit_msg = commit_hash_to_msg[commit_hashes[dataIdx]]
        tooltip.textContent =
            fmtDate(new Date(u.data[0][dataIdx] * 1e3)) +
            '\n' +
            commit_msg.slice(0, 170) +
            '\n' +
            uPlot.fmtNum(u.data[seriesIdx][dataIdx]) +
            ' (' +
            pctSinceStart +
            '% since start)'

        let width = tooltip.getBoundingClientRect().width
        tooltip.style.left = tooltipLeftOffset + lft + shiftY - width / 2 + 'px'
    }

    return {
        hooks: {
            ready: [
                (u) => {
                    over = u.over
                    tooltipLeftOffset = parseFloat(over.style.left)
                    tooltipTopOffset = parseFloat(over.style.top)
                    u.root.querySelector('.u-wrap').appendChild(tooltip)

                    let clientX
                    let clientY

                    over.addEventListener('mousedown', (e) => {
                        clientX = e.clientX
                        clientY = e.clientY
                    })

                    over.addEventListener('mouseup', (e) => {
                        // clicked in-place
                        if (e.clientX == clientX && e.clientY == clientY) {
                            if (seriesIdx != null && dataIdx != null) {
                                onclick(u, seriesIdx, dataIdx)
                            }
                        }
                    })
                },
            ],
            setCursor: [
                (u) => {
                    let c = u.cursor

                    if (dataIdx != c.idx) {
                        dataIdx = c.idx

                        if (seriesIdx != null) {
                            setTooltip(u)
                        }
                    }
                },
            ],
            setSeries: [
                (u, sidx) => {
                    if (seriesIdx != sidx) {
                        seriesIdx = sidx

                        if (sidx == null) hideTooltip()
                        else if (dataIdx != null) setTooltip(u)
                    }
                },
            ],
            drawAxes: [
                (u) => {
                    let { ctx } = u

                    const interpolatedColorWithAlpha = '#fcb0f17a'

                    ctx.save()

                    ctx.strokeStyle = interpolatedColorWithAlpha
                    ctx.beginPath()

                    ctx.closePath()
                    ctx.stroke()
                    ctx.restore()
                },
            ],
        },
    }
}

function genPlotOpts({
    title,
    width,
    height,
    yAxisLabel,
    series,
    alpha = 0.3,
    prox = 5,
    absoluteMode,
    commit_hashes,
    commit_hash_to_msg,
}) {
    return {
        title,
        width,
        height,
        series,
        legend: { live: false },
        focus: { alpha },
        cursor: {
            focus: { prox },
            drag: { x: true, y: true },
        },
        scales: {
            y: {
                range: (self, dataMin, dataMax) =>
                    uPlot.rangeNum(
                        absoluteMode ? 0 : dataMin,
                        dataMax,
                        0.2,
                        true
                    ),
            },
        },
        axes: [
            {
                grid: {
                    show: false,
                },
            },
            {
                label: yAxisLabel,
                space: 24,
                values: (self, splits) => {
                    return splits.map((v) => {
                        return v >= 1e12
                            ? v / 1e12 + 'T'
                            : v >= 1e9
                            ? v / 1e9 + 'G'
                            : v >= 1e6
                            ? v / 1e6 + 'M'
                            : v >= 1e3
                            ? v / 1e3 + 'k'
                            : v
                    })
                },
            },
        ],
        plugins: [
            tooltipPlugin({
                onclick(u, seriesIdx, dataIdx) {
                    let thisCommit = commit_hashes[dataIdx]
                    let prevCommit = commit_hashes[dataIdx - 1] || [null, null]
                    alert(
                        'thisCommit:' + thisCommit + ' prevCommit:' + prevCommit
                    )
                    //window.open(`/compare.html?start=${prevCommit}&end=${thisCommit}`);
                },
                commit_hashes,
                commit_hash_to_msg,
                absoluteMode,
            }),
        ],
    }
}

function makeChart(title, data, commit_hashes, commit_hash_to_msg) {
    console.time('chart')

    let yAxisLabel = 'Time in ns'
    let series = [
        {},
        { label: 'NS', stroke: 'red' },
        { label: 'Variance', stroke: 'blue' },
    ]

    let plotOpts = genPlotOpts({
        title,
        width: Math.floor(window.innerWidth / 3) - 16,
        height: 300,
        yAxisLabel,
        series,
        absoluteMode: true,
        commit_hashes,
        commit_hash_to_msg,
    })

    let uplot = new uPlot(plotOpts, data, document.body)

    console.timeEnd('chart')
}

function get_ts_or_commit(ts_or_commit) {
    let date = new Date(ts_or_commit)
    if (date.toString() != 'Invalid Date') {
        return date.getTime() / 1000
    }
    return ts_or_commit
}

function loadData(state) {
    let d = new Date()
    d.setDate(d.getDate() - 30)
    let ts_30_days_ago = start || d.getTime() / 1000

    let start = get_ts_or_commit(state.start) || d.getTime() / 1000
    let end = get_ts_or_commit(state.end)
    let wait = document.getElementById('wait')
    wait.textContent = 'Fetching data.json....'

    fetch('data.json')
        .then((r) => r.json())
        .then((data) => {
            let commit_hash_to_msg = data['commit_hash_to_message']
            let benchmarks = data['benchmarks']
            wait.textContent = 'Rendering...'
            for (let benchmark of benchmarks) {
                let data1 = filterData(
                    benchmark['uplot_data'],
                    benchmark['commit_hashes'],
                    ts_30_days_ago,
                    end
                )
                let data = data1.data
                let commit_hashes = data1.commit_hashes
                if (data && data[0].length > 2) {
                    makeChart(
                        benchmark.name,
                        data,
                        commit_hashes,
                        commit_hash_to_msg
                    )
                }
            }
            wait.textContent = ''
        })
}

/// Filter out data by timestamp (e.g. last 30 days)
function filterData(data, commit_hashes, min_ts, max_ts) {
    let cut_num_front = 0
    let timestamps = data[0]
    let end_pos = timestamps && timestamps.len && timestamps.len()
    for (let i = 0, len = timestamps.length; i < len; i++) {
        const timestamp = timestamps[i]
        if (timestamp < min_ts) {
            cut_num_front++
        }
        if (max_ts && max_ts < timestamp) {
            end_pos = i
            break
        }
    }
    return {
        data: [
            data[0].slice(cut_num_front, end_pos),
            data[1].slice(cut_num_front, end_pos),
            data[2].slice(cut_num_front, end_pos),
        ],
        commit_hashes: commit_hashes.slice(cut_num_front, end_pos),
    }
}
