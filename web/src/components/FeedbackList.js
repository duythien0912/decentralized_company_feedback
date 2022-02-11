// src/components/FeedbackList.js
import { useEffect, useState } from "react";
import { Feedback } from "./Feedback";

const PER_PAGE_LIMIT = 3;

const FeedbackList = ({ contract }) => {
  const [feedbacks, setFeedbacks] = useState([]);
  const [page, setPage] = useState(1);

  useEffect(() => {
    let offset;
    if (page < 1) {
      setPage(1);
      offset = 0;
    } else {
      offset = (page - 1) * PER_PAGE_LIMIT;
    }

    // every second after the component first mounts
    // update the list of feedbacks by invoking the get
    // method on the smart contract
    const id = setInterval(() => {
      contract
        .get_feedbacks({ page: offset, size: PER_PAGE_LIMIT })
        .then((feedbacks) => {
          console.log("get_feedbacks::", feedbacks);
          return setFeedbacks(feedbacks);
        });
    }, 1000);

    return () => clearInterval(id);
  }, [page, contract]);

  return (
    <ul>
      {feedbacks.map((feedback) => (
        <li key={feedback.id}>
          <Feedback contract={contract} {...feedback} />
        </li>
      ))}
      <hr />
      <div className="flex">Current Page: {page}</div>
      <button onClick={() => setPage((page) => page - 1)}>&lt;</button>{" "}
      <button onClick={() => setPage((page) => page + 1)}>&gt;</button>
    </ul>
  );
};

export default FeedbackList;
