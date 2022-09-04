namespace UIWidgets
{
	using System;
	using System.Collections.Generic;
	using System.Collections.ObjectModel;
	using System.Runtime.CompilerServices;
	using System.Threading.Tasks;
	using UIWidgets.Styles;
	using UnityEngine;
	using UnityEngine.UI;

	/// <summary>
	/// Base class for Pickers.
	/// </summary>
	/// <typeparam name="TValue">Type of value.</typeparam>
	/// <typeparam name="TPicker">Type of picker.</typeparam>
	public abstract class Picker<TValue, TPicker> : MonoBehaviour, ITemplatable, IStylable, INotifyCompletion
		where TPicker : Picker<TValue, TPicker>
	{
		/// <summary>
		/// Picker result.
		/// </summary>
		public struct Result
		{
			/// <summary>
			/// Selected value or default value if nothing selected.
			/// </summary>
			public readonly TValue Value;

			/// <summary>
			/// True if value was selected; otherwise false.
			/// </summary>
			public readonly bool Success;

			/// <summary>
			/// Initializes a new instance of the <see cref="Result"/> struct.
			/// </summary>
			/// <param name="value">Value.</param>
			/// <param name="success">True if value was selected; otherwise false.</param>
			public Result(TValue value, bool success = true)
			{
				Value = value;
				Success = success;
			}

			/// <summary>
			/// Convert this instance to value.
			/// </summary>
			/// <param name="result">Picker result.</param>
			public static implicit operator TValue(Result result)
			{
				return result.Value;
			}

			/// <summary>
			/// Returns a string that represents the selected value.
			/// </summary>
			/// <returns>A string that represents the selected value.</returns>
			public override string ToString()
			{
				return Value.ToString();
			}
		}

		bool isTemplate = true;

		/// <summary>
		/// Gets a value indicating whether this instance is template.
		/// </summary>
		/// <value><c>true</c> if this instance is template; otherwise, <c>false</c>.</value>
		public bool IsTemplate
		{
			get
			{
				return isTemplate;
			}

			set
			{
				isTemplate = value;
			}
		}

		/// <summary>
		/// Gets the name of the template.
		/// </summary>
		/// <value>The name of the template.</value>
		public string TemplateName
		{
			get;
			set;
		}

		static Templates<TPicker> templates;

		/// <summary>
		/// Picker templates.
		/// </summary>
		[System.Diagnostics.CodeAnalysis.SuppressMessage("Microsoft.Design", "CA1000:DoNotDeclareStaticMembersOnGenericTypes", Justification = "Reviewed")]
		public static Templates<TPicker> Templates
		{
			get
			{
				if (templates == null)
				{
					templates = new Templates<TPicker>();
				}

				return templates;
			}

			set
			{
				templates = value;
			}
		}

		[SerializeField]
		Button closeButton;

		/// <summary>
		/// Close button.
		/// </summary>
		public Button CloseButton
		{
			get
			{
				return closeButton;
			}

			set
			{
				if (isInited && (closeButton != null))
				{
					closeButton.onClick.RemoveListener(Cancel);
				}

				closeButton = value;

				if (isInited && (closeButton != null))
				{
					closeButton.onClick.AddListener(Cancel);
				}
			}
		}

		/// <summary>
		/// Opened pickers.
		/// </summary>
		protected static HashSet<TPicker> openedPickers = new HashSet<TPicker>();

		/// <summary>
		/// List of the opened pickers.
		/// </summary>
		protected static List<TPicker> OpenedPickersList = new List<TPicker>();

		/// <summary>
		/// Opened pickers.
		/// </summary>
		public static ReadOnlyCollection<TPicker> OpenedPickers
		{
			get
			{
				OpenedPickersList.Clear();
				OpenedPickersList.AddRange(openedPickers);

				return OpenedPickersList.AsReadOnly();
			}
		}

		/// <summary>
		/// Inactive pickers with the same template.
		/// </summary>
		public List<TPicker> InactivePickers
		{
			get
			{
				return Templates.CachedInstances(TemplateName);
			}
		}

		/// <summary>
		/// All pickers.
		/// </summary>
		public static List<TPicker> AllPickers
		{
			get
			{
				var pickers = Templates.GetAll();
				pickers.AddRange(OpenedPickers);

				return pickers;
			}
		}

		/// <summary>
		/// Count of the opened pickers.
		/// </summary>
		public static int Opened
		{
			get
			{
				return openedPickers.Count;
			}
		}

		/// <summary>
		/// Is instance destroyed?
		/// </summary>
		public bool IsDestroyed
		{
			get;
			protected set;
		}

		/// <summary>
		/// Get opened pickers.
		/// </summary>
		/// <param name="output">Output list.</param>
		public static void GetOpenedPickers(List<TPicker> output)
		{
			output.AddRange(openedPickers);
		}

		/// <summary>
		/// Process the enable event.
		/// </summary>
		protected virtual void OnEnable()
		{
			if (!IsTemplate)
			{
				openedPickers.Add(this as TPicker);
			}
		}

		/// <summary>
		/// Process the disable event.
		/// </summary>
		protected virtual void OnDisable()
		{
			if (!IsTemplate)
			{
				openedPickers.Remove(this as TPicker);
			}
		}

		/// <summary>
		/// Awake is called when the script instance is being loaded.
		/// </summary>
		protected virtual void Awake()
		{
			if (IsTemplate)
			{
				gameObject.SetActive(false);
			}
		}

		/// <summary>
		/// Start this instance.
		/// </summary>
		protected virtual void Start()
		{
			Init();
		}

		bool isInited;

		/// <summary>
		/// Init this instance.
		/// </summary>
		public virtual void Init()
		{
			if (isInited)
			{
				return;
			}

			if (closeButton != null)
			{
				closeButton.onClick.AddListener(Cancel);
			}

			isInited = true;
		}

		/// <summary>
		/// This function is called when the MonoBehaviour will be destroyed.
		/// </summary>
		protected virtual void OnDestroy()
		{
			if (!IsTemplate)
			{
				if (gameObject.activeSelf)
				{
					IsDestroyed = true;
					Cancel();
				}

				templates = null;
				return;
			}

			// if FindTemplates never called than TemplateName == null
			if (TemplateName != null)
			{
				Templates.Delete(TemplateName);
			}
		}

		/// <summary>
		/// Return picker instance using current instance as template.
		/// </summary>
		/// <returns>New picker instance.</returns>
		[Obsolete("Use Clone() instead.")]
		public TPicker Template()
		{
			return Clone();
		}

		/// <summary>
		/// Return picker instance using current instance as template.
		/// </summary>
		/// <returns>New picker instance.</returns>
		public virtual TPicker Clone()
		{
			var picker = this as TPicker;
			if ((TemplateName != null) && Templates.Exists(TemplateName))
			{
				// do nothing
			}
			else if (!Templates.Exists(gameObject.name))
			{
				Templates.Add(gameObject.name, picker);
			}
			else if (Templates.Get(gameObject.name) != this)
			{
				Templates.Add(gameObject.name, picker);
			}

			var id = gameObject.GetInstanceID().ToString();
			if (!Templates.Exists(id))
			{
				Templates.Add(id, picker);
			}
			else if (Templates.Get(id) != this)
			{
				Templates.Add(id, picker);
			}

			return Templates.Instance(id);
		}

		/// <summary>
		/// The modal ID.
		/// </summary>
		protected InstanceID? ModalKey;

		/// <summary>
		/// Callback with selected value.
		/// </summary>
		protected Action<TValue> OnSelect;

		/// <summary>
		/// Callback when picker closed without any value selected.
		/// </summary>
		protected Action OnCancel;

		/// <summary>
		/// Show picker.
		/// </summary>
		/// <param name="defaultValue">Default value.</param>
		/// <param name="onSelect">Callback with selected value.</param>
		/// <param name="onCancel">Callback when picker closed without any value selected.</param>
		/// <param name="modalSprite">Modal sprite.</param>
		/// <param name="modalColor">Modal color.</param>
		/// <param name="canvas">Canvas.</param>
		public virtual void Show(
			TValue defaultValue = default(TValue),
			Action<TValue> onSelect = null,
			Action onCancel = null,
			Sprite modalSprite = null,
			Color? modalColor = null,
			Canvas canvas = null)
		{
			AsyncResult = new Result(defaultValue, false);
			if (modalColor == null)
			{
				modalColor = new Color(0, 0, 0, 0.8f);
			}

			if (IsTemplate)
			{
				Debug.LogWarning("Use the template clone, not the template itself: PickerTemplate.Clone().Show(...), not PickerTemplate.Show(...)");
			}

			OnSelect = onSelect;
			OnCancel = onCancel;

			var canvas_rt = SetCanvas(canvas);
			SetModal(modalSprite, modalColor, canvas_rt);

			gameObject.SetActive(true);

			BeforeOpen(defaultValue);
		}

		/// <summary>
		/// Show picker.
		/// </summary>
		/// <param name="defaultValue">Default value.</param>
		/// <param name="modalSprite">Modal sprite.</param>
		/// <param name="modalColor">Modal color.</param>
		/// <param name="canvas">Canvas.</param>
		/// <returns>Selected value and confirmation.</returns>
		public async virtual Task<Result> ShowAsync(
			TValue defaultValue = default(TValue),
			Sprite modalSprite = null,
			Color? modalColor = null,
			Canvas canvas = null)
		{
			Show(defaultValue, modalSprite: modalSprite, modalColor: modalColor, canvas: canvas);
			return await this;
		}

		/// <summary>
		/// Set modal mode.
		/// Warning: modal block is created at the current root canvas.
		/// </summary>
		/// <param name="modalSprite">Modal sprite.</param>
		/// <param name="modalColor">Modal color.</param>
		/// <param name="parentCanvas">Parent canvas.</param>
		public virtual void SetModal(Sprite modalSprite = null, Color? modalColor = null, RectTransform parentCanvas = null)
		{
			if (ModalKey != null)
			{
				ModalHelper.Close(ModalKey.Value);
				ModalKey = null;
			}

			ModalKey = ModalHelper.Open(this, modalSprite, modalColor, parent: parentCanvas);

			transform.SetAsLastSibling();
		}

		/// <summary>
		/// Set canvas.
		/// </summary>
		/// <param name="canvas">Canvas.</param>
		/// <returns>Canvas RectTransform.</returns>
		public virtual RectTransform SetCanvas(Canvas canvas)
		{
			return SetCanvas(canvas != null ? canvas.transform as RectTransform : null);
		}

		/// <summary>
		/// Set canvas.
		/// </summary>
		/// <param name="parent">Parent.</param>
		/// <returns>Canvas RectTransform.</returns>
		public virtual RectTransform SetCanvas(RectTransform parent)
		{
			if (parent == null)
			{
				parent = UtilitiesUI.FindTopmostCanvas(gameObject.transform);
			}

			if (parent != null)
			{
				transform.SetParent(parent, false);
			}

			transform.SetAsLastSibling();

			return parent;
		}

		/// <summary>
		/// Close picker with specified value.
		/// </summary>
		/// <param name="value">Value.</param>
		public virtual void Selected(TValue value)
		{
			if (OnSelect != null)
			{
				OnSelect.Invoke(value);
			}

			AsyncResult = new Result(value);
			Complete();
			Close();
		}

		/// <summary>
		/// Close picker without specified value.
		/// </summary>
		public virtual void Cancel()
		{
			if (OnCancel != null)
			{
				OnCancel();
			}

			Complete();
			Close();
		}

		/// <summary>
		/// Prepare picker to open.
		/// </summary>
		/// <param name="defaultValue">Default value.</param>
		public abstract void BeforeOpen(TValue defaultValue);

		/// <summary>
		/// Prepare picker to close.
		/// </summary>
		public abstract void BeforeClose();

		/// <summary>
		/// Close picker.
		/// </summary>
		protected virtual void Close()
		{
			BeforeClose();

			if (ModalKey.HasValue)
			{
				ModalHelper.Close(ModalKey.Value);
			}

			Return();
		}

		/// <summary>
		/// Return this instance to cache.
		/// </summary>
		protected virtual void Return()
		{
			Templates.ToCache(this as TPicker);
		}

		#region async

		Action Continuation;

		/// <summary>
		/// Async result.
		/// </summary>
		protected Result AsyncResult;

		/// <summary>
		/// Gets a value that indicates whether the asynchronous task has completed.
		/// </summary>
		public virtual bool IsCompleted
		{
			get;
			protected set;
		}

		/// <summary>
		/// Gets an awaiter used to await this result.
		/// </summary>
		/// <returns>Awaiter.</returns>
		public virtual TPicker GetAwaiter()
		{
			IsCompleted = false;
			return this as TPicker;
		}

		/// <summary>
		/// Ends the wait for the completion of the asynchronous task.
		/// </summary>
		/// <returns>Result.</returns>
		public virtual Result GetResult()
		{
			return AsyncResult;
		}

		/// <summary>
		/// Sets the action to perform when the this object stops waiting for the asynchronous task to complete.
		/// </summary>
		/// <param name="continuation">The action to perform when the wait operation completes.</param>
		public void OnCompleted(Action continuation)
		{
			Continuation = continuation;
		}

		/// <summary>
		/// Complete asynchronous task.
		/// </summary>
		protected void Complete()
		{
			if (Continuation != null)
			{
				IsCompleted = true;

				var c = Continuation;
				Continuation = null;
				c?.Invoke();
			}
		}

		#endregion

		#region IStylable implementation

		/// <inheritdoc/>
		public virtual bool SetStyle(Style style)
		{
			style.Dialog.Background.ApplyTo(GetComponent<Image>());
			style.Dialog.Title.ApplyTo(transform.Find("Header/Title"));

			style.Dialog.ContentBackground.ApplyTo(transform.Find("Content"));

			style.Dialog.Delimiter.ApplyTo(transform.Find("Delimiter/Delimiter"));

			if (closeButton != null)
			{
				style.ButtonClose.ApplyTo(closeButton);
			}
			else
			{
				style.ButtonClose.Background.ApplyTo(transform.Find("Header/CloseButton"));
				style.ButtonClose.Text.ApplyTo(transform.Find("Header/CloseButton/Text"));
			}

			return true;
		}

		/// <inheritdoc/>
		public virtual bool GetStyle(Style style)
		{
			style.Dialog.Background.GetFrom(GetComponent<Image>());
			style.Dialog.Title.GetFrom(transform.Find("Header/Title"));

			style.Dialog.ContentBackground.GetFrom(transform.Find("Content"));

			style.Dialog.Delimiter.GetFrom(transform.Find("Delimiter/Delimiter"));

			if (closeButton != null)
			{
				style.ButtonClose.GetFrom(closeButton);
			}
			else
			{
				style.ButtonClose.Background.GetFrom(transform.Find("Header/CloseButton"));
				style.ButtonClose.Text.GetFrom(transform.Find("Header/CloseButton/Text"));
			}

			return true;
		}

		#endregion
	}
}