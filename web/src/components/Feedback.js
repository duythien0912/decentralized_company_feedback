// src/components/Feedback.js

export function Feedback({ contract, id, content, create_at }) {
  return (
    <>
      <p>
        {id}. {content} - {new Date(create_at / 1000000).toLocaleString()}
      </p>
    </>
  );
}
