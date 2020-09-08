import React, { useCallback, useState } from "react";
import AsyncSelect from "react-select/async";
import IArtist from "./types/IArtist";
import DatabaseService from "./services/DatabaseService";
import { debounce } from "lodash";
import ICalculation from "./types/ICalculation";
import BackgroundGrid from "./components/backgroundGrid/BackgroundGrid";
import "./App.css";
import Grid from "./components/grid/Grid";

type Voidable<T> = T | null | undefined;

const BRIAN_ENO: IArtist = {
  id: 21240,
  name: "Brian Eno",
};

function App() {
  const [start, setStart] = useState<Voidable<IArtist>>();
  const [end, setEnd] = useState<Voidable<IArtist>>(BRIAN_ENO);

  const [calcResult, setCalcResult] = useState<ICalculation>();

  const onLoadOptions = useCallback(
    debounce((query: string, callback) => {
      if (query?.length >= 2) {
        DatabaseService.get().searchArtists(query).then(callback);
        return;
      }

      callback([]);
    }, 250),
    []
  );

  const onCalculate = useCallback(() => {
    if (start && end) {
      DatabaseService.get().calculate(start, end).then(setCalcResult);
    }
  }, [start, end]);

  return (
    <div>
      <BackgroundGrid />
      <div style={{position: "fixed", zIndex: 10}}>
        <AsyncSelect
          placeholder={"Start"}
          loadOptions={onLoadOptions}
          getOptionValue={(option) => option.id.toString()}
          getOptionLabel={(option) => option.name}
          value={start}
          onChange={(newStart) => setStart(newStart as IArtist)}
          cacheOptions
          escapeClearsValue
        />
        <AsyncSelect
          placeholder={"Destination"}
          loadOptions={onLoadOptions}
          getOptionValue={(option) => option.id.toString()}
          getOptionLabel={(option) => option.name}
          value={end}
          onChange={(newEnd) => setEnd(newEnd as IArtist)}
          cacheOptions
          escapeClearsValue
        />
        <button onClick={onCalculate}>Calculate</button>
        <div>Start: {start?.name}</div>
        <div>Destination: {end?.name}</div>
        <div>Time: {calcResult?.time}</div>
      </div>
        <Grid artists={calcResult?.path || []} />
    </div>
  );
}

export default App;
