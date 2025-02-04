



[Mission 3 - Audio Essentials: In-Editor - Unity Learn](https://learn.unity.com/mission/mission-3-audio-essentials-in-editor?uv=6%20preview&labelRequired=true&pathwayId=66c4af96edbc2a1604fdfba1)

默认情况下，Main Camera 会带有一个 *Audio Listener* 组件，相当于玩家在游戏世界的“耳朵。

对应着，可以对不同的 Game Object 添加 *Audio Source* 组件，作为声音源。

此外，可以通过 *Audio Reverb Zone* 组件来控制混响。

## Audio Source

**Loop**：播放完毕后是否循环

**Play On Awake**：是否在 Component 加载时播放

**Spatial Blend**：

2D 声音与 3D 声音是不同的，后者受 *Audio Listener* 的位置与朝向影响，而前者不会，因此前者通常用于背景音乐，而后者常用于游戏物体的音效。通过*Audio Source* 的 Spatial Blend 属性可以调整 2D 与 3D 的混合，默认情况下此值为 0，即完全 2D。

### PlaySoundAtRandomIntervals

```c#
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PlaySoundAtRandomIntervals : MonoBehaviour
{
    public float minSeconds = 5f; // Minimum interval to wait before playing sound.
    public float maxSeconds = 15f; // Maximum interval to wait before playing sound.

    private AudioSource audioSource;

    private void Start()
    {
        audioSource = GetComponent<AudioSource>();
        StartCoroutine(PlaySound());
    }

    private IEnumerator PlaySound()
    {
        while (true)
        {
            float waitTime = Random.Range(minSeconds, maxSeconds);
            yield return new WaitForSeconds(waitTime);
            audioSource.Play();
        }
    }
}
```

## Scripting

### MonoBehaviour Script

```rust
using UnityEngine;

public class PlayerController : MonoBehaviour
{
    // Start is called once before the first execution of Update after the MonoBehaviour is created
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}

```

```c#
using UnityEngine;

// Controls player movement and rotation.
public class PlayerController : MonoBehaviour
{
    public float speed = 5.0f; // Set player's movement speed.
    public float rotationSpeed = 120.0f; // Set player's rotation speed.
    public float jumpForce = 5.0f;

    private Rigidbody rb; // Reference to player's Rigidbody.

    // Start is called before the first frame update
    private void Start()
    {
        rb = GetComponent<Rigidbody>(); // Access player's Rigidbody.
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetButtonDown("Jump")) {
            rb.AddForce(Vector3.up * jumpForce, ForceMode.VelocityChange);
        }
    }

    // Handle physics-based movement and rotation.
    private void FixedUpdate()
    {
        // Move player based on vertical input.
        float moveVertical = Input.GetAxis("Vertical");
        Vector3 movement = transform.forward * moveVertical * speed * Time.fixedDeltaTime;
        rb.MovePosition(rb.position + movement);

        // Rotate player based on horizontal input.
        float turn = Input.GetAxis("Horizontal") * rotationSpeed * Time.fixedDeltaTime;
        Quaternion turnRotation = Quaternion.Euler(0f, turn, 0f);
        rb.MoveRotation(rb.rotation * turnRotation);
    }
}

```





```c#
using UnityEngine;

public class Collectible : MonoBehaviour
{
    public float rotationSpeed;
    public GameObject onCollectEffect;

    // Start is called once before the first execution of Update after the MonoBehaviour is created
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        transform.Rotate(0, rotationSpeed, 0);
    }

    private void OnTriggerEnter(Collider other) {
        if (other.CompareTag("Player")) {
            Destroy(gameObject);
            Instantiate(onCollectEffect, transform.position, transform.rotation);
        }
    }
}

```





```c#
using UnityEngine;


public class DoorOpener : MonoBehaviour
{
   private Animator doorAnimator;

   void Start()
   {
       // Get the Animator component attached to the same GameObject as this script
       doorAnimator = GetComponent<Animator>();
   }

   private void OnTriggerEnter(Collider other)
   {
       // Check if the object entering the trigger is the player (or another specified object)
       if (other.CompareTag("Player")) // Make sure the player GameObject has the tag "Player"
       {
           if (doorAnimator != null)
           {
               // Trigger the Door_Open animation
               doorAnimator.SetTrigger("Door_Open");
           }
       }
   }
}

```

