const Team = Object.freeze({Orange: "Orange", Blue: "Blue"});

const configuration = {
    position: 'start',
    draggable: true,
    pieceTheme: 'img/figurines/{piece}.png',
    onDrop: dropHandler,
};

class WorldState {
    constructor() {
        this.initiative = Team.Orange;
        this.preservedServiceEligibilities = "KQkq";
        this.multifield = ChessBoard('world', configuration);
    }

    preserveInitiative() {
        if (this.initiative === Team.Orange) {
            return 'w';
        } else if (this.initiative === Team.Blue) {
            return 'b';
        }
    }

    preserve() {
        return `${this.multifield.fen()} ` +
            `${this.preserveInitiative()} ` +
            `${this.preservedServiceEligibilities}`;
    }

    cedeInitiative() {
        this.initiative = (this.initiative == Team.Orange) ? Team.Blue : Team.Orange;
    }
}

let world = new WorldState();

function guiAgentRuneToIcon(agentRune) {
    return $('<img>').attr('src', `img/figurines/${agentRune}.png`)
        .addClass('agent-icon');
}

function leaflineAgentToGuiAgentRune(agent) {
    let teamToPrefix = {'Orange': "w", 'Blue': "b"};
    let jobDescriptionToTail = {
        'Servant': "P",
        'Pony': "N",
        'Scholar': "B",
        'Cop': "R",
        'Princess': "Q",
        'Figurehead': "K",
    };
    return (teamToPrefix[agent.team] +
            jobDescriptionToTail[agent.job_description]);
}

function leaflineAgentToIcon(agent) {
    return guiAgentRuneToIcon(leaflineAgentToGuiAgentRune(agent));
}

function localeToAlgebraic(locale) {
    let rank = String.fromCharCode(locale.rank + 49);  // 49 == '1'
    let file = String.fromCharCode(locale.file + 97);  // 97 == 'a'
    return file + rank;
}

function sendPostcard(news) {
    console.log(`sending postcard about ${news}`);
    $.ajax({
        url: "/write/",
        method: 'POST',
        data: {
            world: world.preserve(),
        },
        success: function (data, textStatus, jqxhr) {
            let [newField, _initiative,
                 newEligibilities] = data.world.split(/ /);
            world.cedeInitiative();
            world.multifield.position(newField);
            world.preservedServiceEligibilities = newEligibilities;
            let commentary = ` (after ${data.thinking_time} ms thinking time)`;
            // XXX we are not really respecting the separation of
            // concerns here; mixing logic and presentation will only
            // create a maintenence burden later
            let hospital_mugshot;
            if (data.hospitalization) {
                hospital_mugshot = leaflineAgentToIcon(data.hospitalization);
            } else {
                hospital_mugshot = null;
            }
            printHeadline(
                "Blue",
                leaflineAgentToIcon(data.patch.star),
                localeToAlgebraic(data.patch.whence),
                localeToAlgebraic(data.patch.whither),
                hospital_mugshot,
                commentary
            );
            transpireYear();
        }
    });
}

function dropHandler(whence, whither, agent,
                     news, _previously, _orientation) {
    if (whence != whither && whither != 'offboard') {
        world.multifield.position(news, false);
        world.cedeInitiative();
        sendPostcard(news);
        printHeadline(
            "Orange", guiAgentRuneToIcon(agent),
            whence, whither, null, null
        );
    }
}

const $history = $('#history');

function getYear() {
    return parseInt($history.attr('data-year'));
}

function transpireYear() {
    $history.attr('data-year', getYear() + 1);
}

function printHeadline(team, figurine, whence, whither,
                       hospitalization, commentary) {
    let year = getYear();
    let $headline = $('<div />').addClass("headline")
        .attr('data-team', team).attr('data-year', year);
    let dateline;
    if (team === "Orange") {
        dateline = year + ". ";
    } else if (team === "Blue") {
        dateline = ".. ";
    }
    $headline.append($('<strong />').text(dateline));

    $headline.append(figurine);
    let newsEvent = ` from ${whence} to ${whither}`;
    $headline.append($('<span />').text(newsEvent));

    if (hospitalization) {
        $headline.append(
            $('<span />').text(`, stunning `)
        );
        $headline.append(hospitalization);
    }

    if (commentary) {
        $headline.append($('<em />').text(commentary).addClass("commentary"));
    }

    $history.append($headline);
}
